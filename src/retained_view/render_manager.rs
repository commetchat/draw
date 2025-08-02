//! Displays touch presses, releases, and cancels.


use bevy::{prelude::*, render::view::RenderLayers};

use crate::{
    RENDER_LAYER_BATCH_STROKES, RENDER_LAYER_RETAINED_IMAGE,
    retained_view::{
        RetainedView,
        camera_change_detection::CameraMovementStatusEvent,
        copy_camera::{RetainedViewEvent, TargetCamera},
    },
};

pub fn render_on_camera_movement_system(
    mut camera_events: EventReader<CameraMovementStatusEvent>,
    mut render_events: EventWriter<RetainedViewEvent>,
) {
    for event in camera_events.read() {
        match event {
            CameraMovementStatusEvent::StartedMoving => {
                render_events.write(RetainedViewEvent::UpdateContinuous);
            }
            CameraMovementStatusEvent::StoppedMoving => {
                render_events.write(RetainedViewEvent::RetainFrame);
            }
        }
    }
}

pub fn handle_render_events_system(
    mut render_events: EventReader<RetainedViewEvent>,
    mut retained_camera: Single<&mut RetainedView>,
) {
    for event in render_events.read() {
        match event {
            RetainedViewEvent::UpdateFrame => {
                match retained_camera.frames_until_disabled.checked_add(2) {
                    Some(new) => {
                        retained_camera.frames_until_disabled = new;
                    }
                    None => (),
                }
            }
            RetainedViewEvent::UpdateContinuous => retained_camera.frames_until_disabled = i32::MAX,
            RetainedViewEvent::RetainFrame => {
                retained_camera.frames_until_disabled = 5;
            }
        }
    }
}

pub fn render_loop(
    mut retained_camera: Single<(&mut RetainedView, &mut Camera)>,
    mut target_camera: Single<(&mut TargetCamera, &mut RenderLayers, &GlobalTransform)>,
) {
    if retained_camera.0.frames_until_disabled == 1 {
        *target_camera.1 = target_camera
            .1
            .clone()
            .without(RENDER_LAYER_BATCH_STROKES)
            .with(RENDER_LAYER_RETAINED_IMAGE);
        retained_camera.1.is_active = true;
    } else if retained_camera.0.frames_until_disabled == 0 {
        retained_camera.1.is_active = false;
    } else if retained_camera.0.frames_until_disabled > 0 {
        retained_camera.1.is_active = false;
        *target_camera.1 = target_camera
            .1
            .clone()
            .with(RENDER_LAYER_BATCH_STROKES)
            .without(RENDER_LAYER_RETAINED_IMAGE);
    }

    retained_camera.0.frames_until_disabled = (retained_camera.0.frames_until_disabled - 1).max(-1);
}
