use bevy::{
    ecs::{
        event::{Event, EventReader, EventWriter},
        query::With,
        system::{Query, Single},
    },
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        touch::{TouchInput, TouchPhase},
    },
    log::info,
    math::{Vec2, Vec2Swizzles},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
    window::CursorMoved,
};

#[derive(Clone, Event)]
pub struct PlayerStateData {
    pub position: Vec2,
    pub zoom: f32,
    pub rotation: f32,
}

use crate::{
    lerp_transform::TargetTransform, player_sprite::player_sprite::PlayerSprite,
    retained_view::copy_camera::TargetCamera,
};

pub fn get_player_state_system(
    mut mouse_motion_events: EventReader<CursorMoved>,
    mut touch_events: EventReader<TouchInput>,
    mut writer: EventWriter<PlayerStateData>,
    camera_query: Single<(&Camera, &GlobalTransform), With<TargetCamera>>,
) {
    let mut position = None;

    let (camera, camera_transform) = *camera_query;

    for event in mouse_motion_events.read() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, event.position) {
            position = Some(world_pos);
        }
    }

    for event in touch_events.read() {
        if event.phase == TouchPhase::Moved {
            continue;
        }

        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, event.position) {
            position = Some(world_pos);
        }
    }

    let position = match position {
        Some(pos) => pos,
        None => return,
    };

    let data = PlayerStateData {
        position: position,
        zoom: camera_transform.scale().x,
        rotation: camera_transform
            .rotation()
            .to_euler(bevy::math::EulerRot::XYZ)
            .2,
    };

    writer.write(data);
}
