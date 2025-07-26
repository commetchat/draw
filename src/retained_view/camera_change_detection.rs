//! Displays touch presses, releases, and cancels.

use bevy::prelude::*;

#[derive(Component, Default)]
pub struct CameraChangeDetector {
    last_position: Vec2,
    last_scale: Vec3,
    last_rotation: Quat,
    did_change_last_frame: bool,
}

#[derive(Event)]
pub enum CameraMovementStatusEvent {
    StartedMoving,
    StoppedMoving,
}

pub fn camera_change_detection_system(
    mut camera_query: Single<(&mut CameraChangeDetector, &GlobalTransform)>,
    mut events: EventWriter<CameraMovementStatusEvent>,
) {
    let camera_pos = camera_query.1.translation();
    let camera_pos = camera_pos.xy();
    let camera_scale = camera_query.1.scale();
    let camera_rotation = camera_query.1.rotation();

    // TODO: Fix precision issues!

    let pos_diff = camera_query.0.last_position - camera_pos;
    let pos_diff = (pos_diff.x).abs() + (pos_diff.y).abs();
    let pos_changed = pos_diff > 0.00007;

    let scale_diff = camera_query.0.last_scale.distance(camera_scale);
    let scale_changed = scale_diff > 0.00001;

    let angle_diff = camera_query.0.last_rotation.angle_between(camera_rotation);
    let angle_changed = camera_rotation != camera_query.0.last_rotation && angle_diff > 0.0001;

    if (pos_changed) {
        info!("Position changed by: {}", pos_diff);
        info!("{} vs {}", camera_query.0.last_position, camera_pos);
    }

    if (scale_changed) {
        info!("Scale changed by: {}", scale_diff);
    }

    if (angle_changed) {
        info!("Angle changed by: {}", angle_diff);
    }

    let has_changed = pos_changed || scale_changed || angle_changed;

    if camera_query.0.did_change_last_frame == true && has_changed == false {
        events.write(CameraMovementStatusEvent::StoppedMoving);
        info!("Camera stopped moving!");
    }

    if camera_query.0.did_change_last_frame == false && has_changed == true {
        events.write(CameraMovementStatusEvent::StartedMoving);
        info!("Camera started moving!");
    }

    camera_query.0.did_change_last_frame = has_changed;
    camera_query.0.last_position = camera_pos;
    camera_query.0.last_scale = camera_scale;
    camera_query.0.last_rotation = camera_rotation;
}
