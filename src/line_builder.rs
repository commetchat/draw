// Port of:
// https://github.com/godotengine/godot/blob/7c472e655f974e1f41ff086fa448c94c220728a2/scene/2d/line_builder.cpp

use std::f32::consts::PI;

use bevy::{
    color::Color,
    log::info,
    math::{
        FloatExt, Rect, Vec2,
        ops::{abs, atan2},
    },
};

#[derive(PartialEq)]
enum LineTextureMode {
    None,
    Tile,
    Stretch,
}

#[derive(PartialEq, Clone)]
pub enum LineCapMode {
    None,
    Box,
    Round,
}

#[derive(PartialEq, Clone)]
pub enum LineJointMode {
    Sharp,
    Bevel,
    Round,
}

#[derive(PartialEq, Copy, Clone)]
enum Orientation {
    Up = 0,
    Down = 1,
}

const CMP_EPSILON: f32 = 0.00001;

pub struct LineBuilder {
    pub points: Vec<Vec2>,
    pub width: f32,
    pub pressures: Vec<f32>,
    texture_mode: LineTextureMode,
    pub begin_cap_mode: LineCapMode,
    pub end_cap_mode: LineCapMode,
    pub joint_mode: LineJointMode,
    pub closed: bool,
    pub default_color: Color,
    _round_precision: i32,
    _last_index: [usize; 2],
    pub vertices: Vec<Vec2>,
    pub colors: Vec<Color>,
    pub uvs: Vec<Vec2>,
    pub indices: Vec<u32>,
    interpolate_color: bool,
}

fn interpolate(r: Rect, v: Vec2) -> Vec2 {
    return Vec2 {
        x: r.min.x.lerp(r.max.x, v.x),
        y: r.min.y.lerp(r.max.y, v.y),
    };
}

fn orthogonal(v: Vec2) -> Vec2 {
    return Vec2 { x: v.y, y: -v.x };
}

impl LineBuilder {
    pub fn new() -> LineBuilder {
        LineBuilder {
            pressures: Vec::new(),
            points: Vec::new(),
            width: 10.0,
            texture_mode: LineTextureMode::None,
            begin_cap_mode: LineCapMode::Round,
            end_cap_mode: LineCapMode::Round,
            joint_mode: LineJointMode::Round,
            closed: false,
            default_color: Color::linear_rgb(1.0, 1.0, 1.0),
            _round_precision: 8,
            _last_index: [0, 0],
            vertices: Vec::new(),
            colors: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
            interpolate_color: false,
        }
    }

    pub fn new_with(points: Vec<Vec2>, pressures: Vec<f32>) -> LineBuilder {
        LineBuilder {
            pressures: pressures,
            points: points,
            width: 10.0,
            texture_mode: LineTextureMode::None,
            begin_cap_mode: LineCapMode::Round,
            end_cap_mode: LineCapMode::Round,
            joint_mode: LineJointMode::Round,
            closed: false,
            default_color: Color::linear_rgb(1.0, 1.0, 1.0),
            _round_precision: 8,
            _last_index: [0, 0],
            vertices: Vec::new(),
            colors: Vec::new(),
            uvs: Vec::new(),
            indices: Vec::new(),
            interpolate_color: false,
        }
    }

    pub fn build(&mut self) {
        let tile_aspect: f32 = 1.0;
        let sharp_limit: f32 = 2.0;

        self.vertices = Vec::<Vec2>::new();
        self.colors = Vec::<Color>::new();
        self.uvs = Vec::<Vec2>::new();
        self.indices = Vec::<u32>::new();
        self._last_index[Orientation::Down as usize] = 0;
        self._last_index[Orientation::Up as usize] = 0;
        self._round_precision = 8;

        if self.points.len() < 2 {
            return;
        }

        assert!(tile_aspect > 0.0);

        let hw = self.width * 0.5;
        let hw_sq = hw * hw;
        let point_count = self.points.len();

        let wrap_around = self.closed && point_count > 2;
        self.interpolate_color = false; // self.gradient.is_some();
        let retrieve_curve = self.pressures.len() == self.points.len(); //self.curve.is_some();

        let distance_required = self.interpolate_color
            || retrieve_curve
            || self.texture_mode == LineTextureMode::Tile
            || self.texture_mode == LineTextureMode::Stretch;

        let mut pos0 = *self.points.get(0).unwrap();
        let mut pos1 = *self.points.get(1).unwrap();
        let mut f0 = (pos1 - pos0).normalize();
        let mut u0 = orthogonal(f0);

        let mut pos_up0 = pos0;
        let mut pos_down0 = pos0;

        let color0 = Color::linear_rgb(1.0, 1.0, 1.0);
        let mut color1 = Color::linear_rgb(1.0, 1.0, 1.0);

        let mut current_distance0 = 0.0;
        let mut current_distance1 = 0.0;
        let mut total_distance = 0.0;

        let mut width_factor = 1.0;
        let mut modified_hw = hw;

        if retrieve_curve {
            width_factor = *self.pressures.first().unwrap(); //1.0; //TODO: Sample curve
            modified_hw = hw * width_factor;
        }

        if distance_required {
            for i in 1..point_count {
                total_distance += self
                    .points
                    .get(i)
                    .unwrap()
                    .distance(*self.points.get(i - 1).unwrap());
            }

            if wrap_around {
                total_distance += self.points.get(point_count - 1).unwrap().distance(pos0);
            } else {
                match self.begin_cap_mode {
                    LineCapMode::Box | LineCapMode::Round => {
                        total_distance += modified_hw;
                    }
                    _ => (),
                }

                match self.end_cap_mode {
                    LineCapMode::Box | LineCapMode::Round => {
                        if retrieve_curve {
                            total_distance += hw * 1.0 //TODO: sample curve
                        } else {
                            total_distance += hw
                        }
                    }
                    _ => (),
                }
            }
        }

        if point_count < 2 || (distance_required && total_distance < 0.0001) {
            return;
        }

        if self.interpolate_color {
            //TODO: Sample Gradient
            //color0 = gradient->get_color(0);
        } else {
            self.colors.push(self.default_color);
        }

        let mut uvx0 = 0.0;
        let mut uvx1 = 0.0;

        pos_up0 += u0 * modified_hw;
        pos_down0 -= u0 * modified_hw;

        if !wrap_around {
            match self.begin_cap_mode {
                LineCapMode::Box => {
                    pos_up0 -= f0 * modified_hw;
                    pos_down0 -= f0 * modified_hw;
                    current_distance0 += modified_hw;
                    current_distance1 = current_distance0;
                }
                LineCapMode::Round => {
                    if self.texture_mode == LineTextureMode::Tile {
                        uvx0 = width_factor * 0.5 / tile_aspect;
                    } else if self.texture_mode == LineTextureMode::Stretch {
                        uvx0 = self.width * width_factor / total_distance;
                    }

                    self.new_arc(
                        pos0,
                        pos_up0 - pos0,
                        -PI,
                        color0,
                        Rect::new(0.0, 0.0, uvx0 * 2.0, 1.0),
                    );

                    current_distance0 += modified_hw;
                    current_distance1 = current_distance0;
                }
                _ => (),
            }

            self.strip_begin(pos_up0, pos_down0, color0, uvx0)
        }

        let segments_count: usize = if wrap_around {
            point_count
        } else {
            point_count - 2
        };

        let first_point: i32 = if wrap_around { -1 } else { 1 };

        let mut first_pos_up = Vec2::ZERO;
        let mut first_pos_down = Vec2::ZERO;
        let mut is_first_joint_sharp = true;
        let pc = i32::try_from(point_count).unwrap();
        for i in first_point..i32::try_from(segments_count).unwrap() {
            pos1 = *self
                .points
                .get(if i == -1 {
                    point_count - 1
                } else {
                    usize::try_from(i % pc).unwrap()
                })
                .unwrap();

            let pos2 = self
                .points
                .get(usize::try_from((i + 1) % pc).unwrap())
                .unwrap();

            let f1 = (pos2 - pos1).normalize();
            let u1 = orthogonal(f1);

            let dp = u0.dot(f1);
            let orientation = if dp > 0.0 {
                Orientation::Up
            } else {
                Orientation::Down
            };

            if distance_required && i >= 1 {
                current_distance1 += pos0.distance(pos1);
            }

            if self.interpolate_color {
                color1 = self.default_color //TODO: Sample gradient  `gradient->get_color_at_offset(current_distance1 / total_distance);`
            }

            if retrieve_curve {
                width_factor = *self
                    .pressures
                    .get(if i == -1 {
                        point_count - 1
                    } else {
                        usize::try_from(i % pc).unwrap()
                    })
                    .unwrap();
                //width_factor = 1.0; // TODO: Sample curve `curve->sample_baked(current_distance1 / total_distance);`
                modified_hw = hw * width_factor;
            }

            let mut inner_normal0 = u0 * modified_hw;
            let mut inner_normal1 = u1 * modified_hw;

            if orientation == Orientation::Down {
                inner_normal0 = -inner_normal0;
                inner_normal1 = -inner_normal1;
            }

            // let mut corner_pos_in = Vec2::ZERO;
            //let mut corner_pos_out = Vec2::ZERO;

            let intersection_result = segment_intersects_segment(
                pos0 + inner_normal0,
                pos1 + inner_normal0,
                pos1 + inner_normal1,
                pos2 + inner_normal1,
            );

            let is_intersecting = intersection_result.0;
            let corner_pos_out;
            let mut corner_pos_in = intersection_result.1;

            if is_intersecting {
                corner_pos_out = 2.0 * pos1 - corner_pos_in;
            } else {
                corner_pos_in = pos1 + inner_normal0;
                corner_pos_out = pos1 - inner_normal0;
            }

            let corner_pos_up;
            let corner_pos_down;
            match orientation {
                Orientation::Up => {
                    corner_pos_up = corner_pos_in;
                    corner_pos_down = corner_pos_out;
                }
                Orientation::Down => {
                    corner_pos_up = corner_pos_out;
                    corner_pos_down = corner_pos_in;
                }
            }

            let mut current_joint_mode = self.joint_mode.clone();
            let pos_up1;
            let pos_down1;
            if is_intersecting {
                let width_factor_sq = width_factor * width_factor;
                if current_joint_mode == LineJointMode::Sharp
                    && corner_pos_out.distance_squared(pos1) / (hw_sq * width_factor_sq)
                        > sharp_limit
                {
                    current_joint_mode = LineJointMode::Bevel;
                }

                if current_joint_mode == LineJointMode::Sharp {
                    pos_up1 = corner_pos_up;
                    pos_down1 = corner_pos_down;
                } else {
                    match orientation {
                        Orientation::Up => {
                            pos_up1 = corner_pos_up;
                            pos_down1 = pos1 - u0 * modified_hw;
                        }
                        Orientation::Down => {
                            pos_up1 = pos1 + u0 * modified_hw;
                            pos_down1 = corner_pos_down;
                        }
                    }
                }
            } else {
                if current_joint_mode == LineJointMode::Sharp {
                    current_joint_mode = LineJointMode::Bevel
                }

                pos_up1 = corner_pos_up;
                pos_down1 = corner_pos_down;
            }

            if self.texture_mode == LineTextureMode::Tile {
                uvx1 = current_distance1 / (self.width * tile_aspect)
            } else if self.texture_mode == LineTextureMode::Stretch {
                uvx1 = current_distance1 / total_distance;
            }

            u0 = u1;
            f0 = f1;
            pos0 = pos1;

            if is_intersecting {
                if current_joint_mode == LineJointMode::Sharp {
                    pos_up0 = pos_up1;
                    pos_down0 = pos_down1;
                } else {
                    match orientation {
                        Orientation::Up => {
                            pos_up0 = corner_pos_up;
                            pos_down0 = pos1 - u1 * modified_hw;
                        }
                        Orientation::Down => {
                            pos_up0 = pos1 + u1 * modified_hw;
                            pos_down0 = corner_pos_down;
                        }
                    }
                }
            } else {
                pos_up0 = pos1 + u1 * modified_hw;
                pos_down0 = pos1 - u1 * modified_hw;
            }

            if i == -1 {
                continue;
            }

            if wrap_around && i == 0 {
                let first_pos_center = (pos_up1 + pos_down1) / 2.0;
                let lerp_factor = 1.0 / width_factor;
                first_pos_up = first_pos_center.lerp(pos_up1, lerp_factor);
                first_pos_down = first_pos_center.lerp(pos_down1, lerp_factor);
                is_first_joint_sharp = current_joint_mode == LineJointMode::Sharp;
            }

            if wrap_around
                && retrieve_curve
                && !is_first_joint_sharp
                && i == i32::try_from(segments_count).unwrap()
            {
                let first_pos_center = (first_pos_up + first_pos_down) / 2.0;
                self.strip_add_quad(
                    first_pos_center.lerp(first_pos_up, width_factor),
                    first_pos_center.lerp(first_pos_down, width_factor),
                    color1,
                    uvx1,
                );
                return;
            } else {
                self.strip_add_quad(pos_up1, pos_down1, color1, uvx1);
            }

            if current_joint_mode != LineJointMode::Sharp {
                let cbegin;
                let cend;
                match orientation {
                    Orientation::Up => {
                        cbegin = pos_down1;
                        cend = pos_down0;
                    }
                    Orientation::Down => {
                        cbegin = pos_up1;
                        cend = pos_up0;
                    }
                }

                if current_joint_mode == LineJointMode::Bevel
                    && !(wrap_around && i == i32::try_from(segments_count).unwrap())
                {
                    self.strip_add_tri(cend, orientation);
                } else if current_joint_mode == LineJointMode::Round
                    && !(wrap_around && i == i32::try_from(segments_count).unwrap())
                {
                    let vbegin = cbegin - pos1;
                    let vend = cend - pos1;

                    let mut cross_product = vbegin.perp_dot(vend); // float cross_product = vbegin.cross(vend);
                    let dot_product = vbegin.dot(vend);

                    if cross_product == -0.0 && cross_product.is_sign_negative() {
                        cross_product = 0.0;
                    }

                    let angle_delta = atan2(cross_product, dot_product);
                    self.strip_add_arc(pos1, angle_delta, orientation);
                }

                if !is_intersecting {
                    self.strip_begin(pos_up0, pos_down0, color1, uvx1);
                }
            }
        }

        if !wrap_around {
            pos1 = *self.points.get(point_count - 1).unwrap();

            if distance_required {
                current_distance1 += pos0.distance(pos1);
            }

            if self.interpolate_color {
                color1 = self.default_color; //TODO: color1 = gradient->get_color(gradient->get_point_count() - 1);
            }

            if retrieve_curve {
                // TODO: width_factor = curve->sample_baked(1.f);

                width_factor = *self.pressures.last().unwrap();

                modified_hw = hw * width_factor;
            }

            let mut pos_up1 = pos1 + u0 * modified_hw;
            let mut pos_down1 = pos1 - u0 * modified_hw;

            if self.end_cap_mode == LineCapMode::Box {
                pos_up1 += f0 * modified_hw;
                pos_down1 += f0 * modified_hw;

                current_distance1 += modified_hw;
            }

            if self.texture_mode == LineTextureMode::Tile {
                uvx1 = current_distance1 / (self.width * tile_aspect);
            } else if self.texture_mode == LineTextureMode::Stretch {
                uvx1 = current_distance1 / total_distance;
            }

            self.strip_add_quad(pos_up1, pos_down1, color1, uvx1);

            if self.end_cap_mode == LineCapMode::Round {
                let color = self.default_color; // TODO: Color color = _interpolate_color ? gradient->get_color(gradient->get_point_count() - 1) : Color(0, 0, 0);
                let dist = match self.texture_mode {
                    LineTextureMode::Tile => width_factor / tile_aspect,
                    LineTextureMode::Stretch => self.width * width_factor / total_distance,
                    _ => 0.0,
                };

                self.new_arc(
                    pos1,
                    pos_up1 - pos1,
                    PI,
                    color,
                    Rect::new(uvx1 - 0.5 * dist, 0.0, dist, 1.),
                );
            }
        }
    }

    fn strip_begin(&mut self, up: Vec2, down: Vec2, color: Color, uvx: f32) {
        let vi = self.vertices.len();

        self.vertices.push(up);
        self.vertices.push(down);

        if self.interpolate_color {
            self.colors.push(color);
            self.colors.push(color);
        }

        if self.texture_mode != LineTextureMode::None {
            self.uvs.push(Vec2::new(uvx, 0.0));
            self.uvs.push(Vec2::new(uvx, 1.0));
        }

        self._last_index[Orientation::Up as usize] = vi;
        self._last_index[Orientation::Down as usize] = vi + 1;
    }

    fn strip_add_quad(&mut self, up: Vec2, down: Vec2, color: Color, uvx: f32) {
        let vi = self.vertices.len();

        self.vertices.push(up);
        self.vertices.push(down);

        if self.interpolate_color {
            self.colors.push(color);
            self.colors.push(color);
        }

        if self.texture_mode != LineTextureMode::None {
            self.uvs.push(Vec2::new(uvx, 0.0));
            self.uvs.push(Vec2::new(uvx, 1.0));
        }

        self.indices
            .push(u32::try_from(self._last_index[Orientation::Up as usize]).unwrap());
        self.indices.push(u32::try_from(vi + 1).unwrap());

        self.indices
            .push(u32::try_from(self._last_index[Orientation::Down as usize]).unwrap());
        self.indices
            .push(u32::try_from(self._last_index[Orientation::Up as usize]).unwrap());
        self.indices.push(u32::try_from(vi).unwrap());
        self.indices.push(u32::try_from(vi + 1).unwrap());

        self._last_index[Orientation::Up as usize] = vi;
        self._last_index[Orientation::Down as usize] = vi + 1;
    }

    fn strip_add_tri(&mut self, up: Vec2, orientation: Orientation) {
        let vi = self.vertices.len();

        self.vertices.push(up);

        if self.interpolate_color {
            self.colors.push(*self.colors.last().unwrap());
        }

        let opposite_direction = match orientation {
            Orientation::Up => Orientation::Down,
            Orientation::Down => Orientation::Up,
        };

        if self.texture_mode != LineTextureMode::None {
            self.uvs.push(
                *self
                    .uvs
                    .get(self._last_index[opposite_direction as usize])
                    .unwrap(),
            );
        }

        self.indices
            .push(u32::try_from(self._last_index[opposite_direction as usize]).unwrap());
        self.indices.push(u32::try_from(vi).unwrap());
        self.indices
            .push(u32::try_from(self._last_index[orientation as usize]).unwrap());

        self._last_index[opposite_direction as usize] = vi;
    }

    fn strip_add_arc(&mut self, center: Vec2, angle_delta: f32, orientation: Orientation) {
        let opposite_direction = match orientation {
            Orientation::Up => Orientation::Down,
            Orientation::Down => Orientation::Up,
        };

        let vbegin = self.vertices[self._last_index[opposite_direction as usize]] - center;

        let radius = vbegin.length();
        let mut angle_step = PI / self._round_precision as f32;
        let steps = abs(angle_delta) / angle_step;

        if angle_delta < 0.0 {
            angle_step = -angle_step;
        }

        let mut t = Vec2::new(1.0, 0.0).angle_to(vbegin);
        let end_angle = t + angle_delta;
        let mut rpos;

        let mut ti = 0;
        while ti < steps as i32 {
            rpos = center + Vec2::new(t.cos(), t.sin()) * radius;
            self.strip_add_tri(rpos, orientation);
            t += angle_step;
            ti += 1;
        }

        rpos = center + Vec2::new(end_angle.cos(), end_angle.sin()) * radius;
        self.strip_add_tri(rpos, orientation);
    }

    fn new_arc(
        &mut self,
        center: Vec2,
        vbegin: Vec2,
        angle_delta: f32,
        color: Color,
        uv_rect: Rect,
    ) {
        let radius = vbegin.length();
        let mut angle_step = PI / self._round_precision as f32;
        let steps = abs(angle_delta) / angle_step;

        if angle_delta < 0.0 {
            angle_step = -angle_step;
        }

        let mut t = Vec2::new(1.0, 0.0).angle_to(vbegin);
        let end_angle = t + angle_delta;
        let mut rpos;
        let tt_begin = -PI / 2.0;
        let mut tt = tt_begin;

        let mut vi = self.vertices.len();
        self.vertices.push(center);

        if self.interpolate_color {
            self.colors.push(color);
        }

        if self.texture_mode != LineTextureMode::None {
            self.uvs.push(interpolate(uv_rect, Vec2 { x: 0.5, y: 0.5 }));
        }

        let mut ti = 0;
        while ti < steps as i32 {
            let sc = Vec2::new(t.cos(), t.sin());
            rpos = center + sc * radius;

            self.vertices.push(rpos);

            if self.interpolate_color {
                self.colors.push(color);
            }

            if self.texture_mode != LineTextureMode::None {
                let tsc = Vec2::new(tt.cos(), tt.sin());
                self.uvs
                    .push(interpolate(uv_rect, 0.5 * (tsc + Vec2::new(1.0, 1.0))));

                tt += angle_step;
            }

            ti += 1;
            t += angle_step;
        }

        let sc = Vec2::new(end_angle.cos(), end_angle.sin());
        rpos = center + sc * radius;
        self.vertices.push(rpos);

        if self.interpolate_color {
            self.colors.push(color);
        }

        if self.texture_mode != LineTextureMode::None {
            tt = tt_begin + angle_delta;
            let tsc = Vec2::new(tt.cos(), tt.sin());
            self.uvs
                .push(interpolate(uv_rect, 0.5 * (tsc + Vec2::new(1.0, 1.0))));
        }

        let vi0 = vi;

        for _ in 0..(steps as i32) {
            self.indices.push(u32::try_from(vi0).unwrap());
            vi += 1;
            self.indices.push(u32::try_from(vi).unwrap());
            self.indices.push(u32::try_from(vi + 1).unwrap());
        }
    }
}

fn segment_intersects_segment(from_a: Vec2, to_a: Vec2, from_b: Vec2, to_b: Vec2) -> (bool, Vec2) {
    let b = to_a - from_a;
    let mut c = from_b - from_a;
    let mut d = to_b - from_a;

    let ablen = b.dot(b);
    if ablen <= 0.0 {
        return (false, Vec2::ZERO);
    }

    let bn = b / ablen;
    c = Vec2::new(c.x * bn.x + c.y * bn.y, c.y * bn.x - c.x * bn.y);
    d = Vec2::new(d.x * bn.x + d.y * bn.y, d.y * bn.x - d.x * bn.y);

    if (c.y < -CMP_EPSILON && d.y < -CMP_EPSILON) || (c.y > CMP_EPSILON && d.y > CMP_EPSILON) {
        return (false, Vec2::ZERO);
    }

    if is_equal_approx(c.y, d.y) {
        return (false, Vec2::ZERO);
    }

    let abpos = d.x + (c.x - d.x) * d.y / (d.y - c.y);

    if (abpos < 0.0) || (abpos > 1.0) {
        return (false, Vec2::ZERO);
    }

    let result = from_a + b * abpos;
    return (true, result);
}

fn is_equal_approx(left: f32, right: f32) -> bool {
    if left == right {
        return true;
    }

    let mut tolerance = CMP_EPSILON * abs(left);
    if tolerance < CMP_EPSILON {
        tolerance = CMP_EPSILON;
    }

    return abs(left - right) < tolerance;
}
