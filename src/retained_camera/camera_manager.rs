//! Displays touch presses, releases, and cancels.

use bevy::{prelude::*, render::view::RenderLayers};

use crate::{
    RENDER_LAYER_BATCH_STROKES, RENDER_LAYER_RETAINED_IMAGE, retained_camera::RetainedCamera,
};

#[derive(Component, Default)]
pub struct CameraMovementRender {
    last_position: Vec2,
    last_scale: Vec3,
    last_rotation: Quat,
}

pub fn camera_render_toggle_system(
    mut camera_query: Single<(
        &mut CameraMovementRender,
        &mut RenderLayers,
        &GlobalTransform,
    )>,
    mut retained_camera: Single<(&mut RetainedCamera, &mut Camera)>,
) {
    let camera_pos = camera_query.2.translation();
    let camera_pos = camera_pos.xy();
    let camera_scale = camera_query.2.scale();
    let camera_rotation = camera_query.2.rotation();

    let should_activate = camera_query.0.last_position.distance(camera_pos) > 0.001
        || camera_query.0.last_scale.distance(camera_scale) > 0.001
        || camera_query.0.last_rotation.angle_between(camera_rotation) > 0.001;

    if retained_camera.0.disable_next_frame {
        retained_camera.1.is_active = false;
        retained_camera.0.disable_next_frame = false;
    }

    if should_activate {
        *camera_query.1 = camera_query
            .1
            .clone()
            .with(RENDER_LAYER_BATCH_STROKES)
            .without(RENDER_LAYER_RETAINED_IMAGE);

        retained_camera.1.is_active = false;
        retained_camera.0.disable_next_frame = false;
    }

    if !should_activate
        && camera_query
            .1
            .intersects(&RenderLayers::layer(RENDER_LAYER_BATCH_STROKES))
            == true
    {
        *camera_query.1 = camera_query
            .1
            .clone()
            .without(RENDER_LAYER_BATCH_STROKES)
            .with(RENDER_LAYER_RETAINED_IMAGE);

        retained_camera.1.is_active = true;
        retained_camera.0.disable_next_frame = true;
    }

    camera_query.0.last_position = camera_pos;
    camera_query.0.last_scale = camera_scale;
    camera_query.0.last_rotation = camera_rotation;
}
