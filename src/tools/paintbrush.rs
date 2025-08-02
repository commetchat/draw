use bevy::{
    color::Color,
    ecs::{
        component::Component,
        entity::Entity,
        event::{EventReader, EventWriter},
        query::With,
        system::{Commands, Single},
    },
    gizmos::gizmos::Gizmos,
    log::info,
    math::Vec2,
    render::camera::Camera,
    transform::components::GlobalTransform,
    window::Window,
};

use crate::{
    active_strokes::active_stroke::{ActiveStrokeEvent, NewPointData, StrokeFinishedData},
    retained_view::copy_camera::TargetCamera,
    stylus_input::StylusEvent,
    tools::tools_plugin::ActiveTool,
    ui::ui_messages::{PaintbrushArgs, ReceivedUIMessage, UIMessage},
    utils::{get_random_uint32, get_system_time},
};

#[derive(Component, Default, Debug)]
pub struct ToolPaintBrush {
    pub args: PaintbrushArgs,
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
    let col = active_paintbrush.0.args.color;
    let pos = active_paintbrush.0.last_pos;

    let mut radius = active_paintbrush.0.args.width / 2.0;
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
            Err(_) => return,
        };

        match event {
            StylusEvent::PointerDown(_) => {
                active_paintbrush.0.is_down = true;
                let timestamp = get_system_time();
                let id_random = get_random_uint32();
                let col = active_paintbrush.0.args.color;

                let width = active_paintbrush.0.args.width * camera_query.1.scale().x;

                let info = NewPointData {
                    timestamp: timestamp,
                    id_random: id_random,
                    color: Color::srgb(col[0], col[1], col[2]),
                    stroke_origin: world_pos,
                    point: world_pos,
                    width: width,
                    owner: None,
                    pressure: data.pressure,
                };

                active_paintbrush.0.current_stroke_info = Some(info.clone());

                stroke_events.write(ActiveStrokeEvent::NewPoint(info));

                info!("Starting new stroke!: {} {}", timestamp, id_random);
            }
            StylusEvent::PointerUp(_) => {
                active_paintbrush.0.is_down = false;
                info!("Stroke finished!");

                match &active_paintbrush.0.current_stroke_info {
                    Some(current) => {
                        stroke_events.write(ActiveStrokeEvent::StrokeFinished(
                            StrokeFinishedData {
                                stroke_origin: current.stroke_origin,
                                timestamp: current.timestamp,
                                id_random: current.id_random,
                                owner: None,
                            },
                        ));
                    }
                    None => (),
                }

                active_paintbrush.0.current_stroke_info = None;
            }
            StylusEvent::PointerMove(_) => match &active_paintbrush.0.current_stroke_info {
                Some(current) => {
                    let delta = active_paintbrush.0.last_pos_screenspace - data.position;
                    if delta.length() < 2.0 {
                        return;
                    }

                    stroke_events.write(ActiveStrokeEvent::NewPoint(NewPointData {
                        stroke_origin: current.stroke_origin,
                        timestamp: current.timestamp,
                        id_random: current.id_random,
                        color: current.color,
                        point: world_pos,
                        width: current.width,
                        owner: None,
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

pub fn paintbrush_ui_system(
    mut paintbrush: Single<(Entity, &mut ToolPaintBrush)>,
    mut events: EventReader<ReceivedUIMessage>,
    mut commands: Commands,
) {
    for event in events.read() {
        match &event.data {
            UIMessage::SetTool(tool) => match tool {
                crate::ui::ui_messages::Tool::Paintbrush(paintbrush_args) => {
                    paintbrush.1.args = paintbrush_args.clone();
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
