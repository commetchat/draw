use bevy::color::ColorToComponents;

use crate::{BACKGROUND, line_builder::LineBuilder, stroke::Stroke};

pub fn timestamp_to_z_offset(timestamp: f64) -> f32 {
    const SECONDS_PER_YEAR: f64 = 31556952.0;
    const START_TIME: f64 = 1740000000.0;
    const END_TIME: f64 = START_TIME + (SECONDS_PER_YEAR * 20.0);

    let z_offset = inverse_lerp(START_TIME, END_TIME, timestamp);

    return z_offset as f32;
}

fn inverse_lerp(a: f64, b: f64, v: f64) -> f64 {
    (v - a) / (b - a)
}

pub fn stroke_to_mesh(stroke: &Stroke) -> (Vec<[f32; 3]>, Vec<[f32; 4]>, Vec<u32>) {
    let pressures = match &stroke.data.pressures {
        Some(pressures) => pressures.clone(),
        None => Vec::new(),
    };

    let mut builder = LineBuilder::new_with(stroke.data.points.clone(), pressures);

    builder.width = stroke.data.width;
    builder.default_color = match stroke.data.stroke_type {
        crate::stroke::StrokeType::Paint(color) => color,
        crate::stroke::StrokeType::Eraser => BACKGROUND,
    };

    let z_offset = timestamp_to_z_offset(stroke.metadata.timestamp);

    let mut colors = Vec::<[f32; 4]>::new();
    let mut vertices = Vec::<[f32; 3]>::new();

    builder.build();

    vertices.reserve(builder.vertices.len());
    colors.reserve(builder.colors.len());

    for p in &builder.vertices {
        vertices.push([
            p.x + stroke.metadata.origin.x,
            p.y + stroke.metadata.origin.y,
            0.0,
        ]);

        let mut color = builder.default_color.to_linear().to_f32_array(); // LinearRgba::from_u8_array_no_alpha().to_f32_array();

        color[3] = z_offset as f32;
        colors.push(color);
    }

    return (vertices, colors, builder.indices);
}
