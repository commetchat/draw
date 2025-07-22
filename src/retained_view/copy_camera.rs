use bevy::{
    ecs::{
        component::Component,
        event::Event,
        query::{With, Without},
        system::Single,
    },
    transform::components::Transform,
};

use crate::retained_view::{RetainedImagePlane, RetainedView};

#[derive(Event, Debug)]
pub enum RetainedViewEvent {
    UpdateFrame,
    UpdateContinuous,
    RetainFrame,
}

#[derive(Component, Default)]
pub struct TargetCamera {}

pub fn copy_camera_system(
    camera_query: Single<
        &Transform,
        (
            With<TargetCamera>,
            Without<RetainedView>,
            Without<RetainedImagePlane>,
        ),
    >,
    mut retained_camera: Single<
        &mut Transform,
        (
            With<RetainedView>,
            Without<RetainedImagePlane>,
            Without<TargetCamera>,
        ),
    >,
    mut plane: Single<
        &mut Transform,
        (
            With<RetainedImagePlane>,
            Without<RetainedView>,
            Without<TargetCamera>,
        ),
    >,
) {
    retained_camera.translation = camera_query.translation;
    retained_camera.scale = camera_query.scale;
    retained_camera.rotation = camera_query.rotation;

    plane.translation = camera_query.translation;
    plane.scale = camera_query.scale;
    plane.rotation = camera_query.rotation;
}
