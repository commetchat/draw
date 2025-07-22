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

    let has_changed = camera_query.0.last_position.distance(camera_pos) > 0.001
        || camera_query.0.last_scale.distance(camera_scale) > 0.001
        || camera_query.0.last_rotation.angle_between(camera_rotation) > 0.001;

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
