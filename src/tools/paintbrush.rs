use bevy::{
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Query, Single},
    },
    gizmos::gizmos::Gizmos,
    log::info,
    math::Vec2,
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::Window,
};

use crate::{
    active_strokes::active_stroke::{
        ActiveStroke, ActiveStrokeEvent, NewPointData, StrokeFinishedData,
    },
    retained_view::copy_camera::TargetCamera,
    stroke::{StrokeSource::User, StrokeType},
    stylus_input::StylusEvent,
    tools::tools_plugin::ActiveTool,
    ui::ui_messages::{PaintbrushArgs, ReceivedUIMessage, Tool, UIMessage},
    user_info::UserInfo,
    utils::{get_random_uint32, get_system_time},
};

#[derive(Component, Default, Debug)]
pub struct ToolPaintBrush {
    pub args: Tool,
    pub last_pos: Vec2,
    pub last_pos_screenspace: Vec2,
    pub last_pressure: f32,
    pub is_down: bool,
    pub current_stroke_info: Option<NewPointData>,
}

pub fn paintbrush_gizmo_system(
    active_paintbrush: Single<(&mut ToolPaintBrush, &mut ActiveTool)>,
    camera_query: Single<(&Camera, &GlobalTransform), With<TargetCamera>>,
    mut gizmos: Gizmos,
) {
    let col = match &active_paintbrush.0.args {
        Tool::LineArt(line_art_args) => {
            line_art_args.color
        },
        Tool::Paintbrush(paintbrush_args) => {
            paintbrush_args.color
        },
        _ => panic!()
    };
    

    let width = match &active_paintbrush.0.args {
        Tool::LineArt(line_art_args) => {
            line_art_args.width
        },
        Tool::Paintbrush(paintbrush_args) => {
            paintbrush_args.width
        },
        _ => panic!()
    };
        
    let pos = active_paintbrush.0.last_pos;

    let mut radius = width / 2.0;
    let zoom = camera_query.1.scale();

    radius *= zoom.x;

    let width_with_pressure = radius * active_paintbrush.0.last_pressure;

    gizmos.circle_2d(pos, radius, Color::srgb(col[0], col[1], col[2]));

    gizmos.circle_2d(
        pos,
        width_with_pressure,
        Color::srgb(col[0], col[1], col[2]),
    );
}

pub fn paintbrush_system(
    mut active_paintbrush: Single<(&mut ToolPaintBrush, &mut ActiveTool)>,
    mut events: EventReader<StylusEvent>,
    mut stroke_events: EventWriter<ActiveStrokeEvent>,
    current_strokes: Query<(Entity, &ActiveStroke)>,
    camera_query: Single<(&Camera, &GlobalTransform), With<TargetCamera>>,
    window: Single<&mut Window>,
) {
    let (camera, camera_transform) = *camera_query;
    let scale = window.resolution.base_scale_factor();

    for event in events.read() {
        let data = match event {
            StylusEvent::PointerMove(pointer_data) => pointer_data,
            StylusEvent::PointerDown(pointer_data) => pointer_data,
            StylusEvent::PointerUp(pointer_data) => pointer_data,
        };

        let world_pos = camera.viewport_to_world_2d(camera_transform, data.position * scale);

        let world_pos = match world_pos {
            Ok(pos) => pos,
            Err(_) => {
                info!("Failed to get world pos!");
                continue;
            }
        };

        match event {
            StylusEvent::PointerDown(_) => {
                if active_paintbrush.0.current_stroke_info.is_some() {
                    finish_stroke(
                        &mut stroke_events,
                        &mut active_paintbrush.0,
                        current_strokes,
                    );
                }

                let col = match &active_paintbrush.0.args {
                    Tool::Paintbrush(paintbrush_args) => paintbrush_args.color,
                    Tool::LineArt(line_art_args) => line_art_args.color,
                    _ => panic!(),
                };

                let col = Color::srgb(col[0], col[1], col[2]);

                let width = match &active_paintbrush.0.args {
                    Tool::Paintbrush(paintbrush_args) => paintbrush_args.width,
                    Tool::LineArt(line_art_args) => line_art_args.width,
                    _ => panic!(),
                };

                active_paintbrush.0.is_down = true;
                let timestamp = get_system_time();
                let id_random = get_random_uint32();

                let width = width * camera_query.1.scale().x;

                let info = NewPointData {
                    timestamp: timestamp,
                    id_random: id_random,
                    stroke_type: match &active_paintbrush.0.args {
                        Tool::Paintbrush(_) => StrokeType::Paint(col),
                        Tool::LineArt(_) => StrokeType::LineArt(col),
                        _ => panic!(),
                    },
                    stroke_origin: world_pos,
                    point: world_pos,
                    width: width,
                    owner: Some(UserInfo::get_user_id()),
                    pressure: data.pressure,
                    source: User,
                };

                active_paintbrush.0.current_stroke_info = Some(info.clone());

                stroke_events.write(ActiveStrokeEvent::NewPoint(info));

                info!("Starting new stroke!: {} {}", timestamp, id_random);
            }
            StylusEvent::PointerUp(_) => {
                info!("Got pointer up!");
                finish_stroke(
                    &mut stroke_events,
                    &mut active_paintbrush.0,
                    current_strokes,
                );
            }
            StylusEvent::PointerMove(_) => match &active_paintbrush.0.current_stroke_info {
                Some(current) => {
                    let delta = active_paintbrush.0.last_pos_screenspace - data.position;
                    if delta.length() < 2.0 {
                        continue;
                    }

                    let col = match &active_paintbrush.0.args {
                        Tool::Paintbrush(paintbrush_args) => paintbrush_args.color,
                        Tool::LineArt(line_art_args) => line_art_args.color,
                        _ => panic!(),
                    };

                    let col = Color::srgb(col[0], col[1], col[2]);

                    stroke_events.write(ActiveStrokeEvent::NewPoint(NewPointData {
                        stroke_origin: current.stroke_origin,
                        timestamp: current.timestamp,
                        id_random: current.id_random,
                        stroke_type: match &active_paintbrush.0.args {
                            Tool::Paintbrush(_) => StrokeType::Paint(col),
                            Tool::LineArt(_) => StrokeType::LineArt(col),
                            _ => panic!(),
                        },
                        point: world_pos,
                        width: current.width,
                        owner: Some(UserInfo::get_user_id()),
                        source: User,
                        pressure: data.pressure,
                    }));
                }
                None => {}
            },
        }

        active_paintbrush.0.last_pos_screenspace = data.position;
        active_paintbrush.0.last_pos = world_pos;
        active_paintbrush.0.last_pressure = data.pressure;
    }
}

fn finish_stroke(
    stroke_events: &mut EventWriter<ActiveStrokeEvent>,
    stroke: &mut ToolPaintBrush,
    current_strokes: Query<(Entity, &ActiveStroke)>,
) {
    stroke.is_down = false;
    info!("Stroke finished!");

    match &stroke.current_stroke_info {
        Some(current) => {
            info!("Owner: {:?}", current.owner);

            for mut active in current_strokes.iter() {
                if active.1.timestamp == current.timestamp
                    && active.1.id_random == current.id_random
                {
                    if active.1.points.len() < 2 {
                        info!("Stroke was empty, removing active stroke without saving");
                        stroke_events.write(ActiveStrokeEvent::DeleteActiveStroke(
                            StrokeFinishedData {
                                stroke_origin: current.stroke_origin,
                                timestamp: current.timestamp,
                                id_random: current.id_random,
                                source: User,
                                owner: current.owner.clone(),
                            },
                        ));

                        stroke.current_stroke_info = None;
                        return;
                    }
                }
            }

            stroke_events.write(ActiveStrokeEvent::StrokeFinished(StrokeFinishedData {
                stroke_origin: current.stroke_origin,
                timestamp: current.timestamp,
                id_random: current.id_random,
                source: User,
                owner: current.owner.clone(),
            }));
        }
        None => {
            info!("No current stroke info!");
        }
    }

    stroke.current_stroke_info = None;
}

pub fn paintbrush_ui_system(
    mut paintbrush: Single<(Entity, &mut ToolPaintBrush)>,
    mut events: EventReader<ReceivedUIMessage>,
    mut commands: Commands,
) {
    for event in events.read() {
        match &event.data {
            UIMessage::SetTool(tool) => match tool {
                Tool::Paintbrush(paintbrush_args) => {
                    paintbrush.1.args = Tool::Paintbrush(paintbrush_args.clone());
                    info!("Received set tool event!, inserting active tool to paintbrush");
                    commands.entity(paintbrush.0).insert_if_new(ActiveTool {});
                }
                Tool::LineArt(lineart_args) => {
                    paintbrush.1.args = Tool::LineArt(lineart_args.clone());
                    info!("Received set tool event!, inserting active tool to paintbrush");
                    commands.entity(paintbrush.0).insert_if_new(ActiveTool {});
                }
                _ => {
                    info!("Received set tool event!, removing active tool from paintbrush");
                    commands.entity(paintbrush.0).remove::<ActiveTool>();
                }
            },
            _ => (),
        }
    }
}
