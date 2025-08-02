use bevy::{
    ecs::{
        component::Component,
        query::With,
        system::{Query, Single},
    },
    math::{Vec2, Vec3},
    render::camera::Camera,
    transform::components::{GlobalTransform, Transform},
};

use crate::retained_view::copy_camera::TargetCamera;

#[derive(Component)]
pub struct ZoomCancel;

pub fn zoom_cancel_system(
    camera_query: Single<(&Camera, &GlobalTransform), With<TargetCamera>>,
    mut sprites: Query<(&ZoomCancel, &mut Transform)>,
) {
    let (camera, camera_transform) = *camera_query;

    for mut sprite in sprites.iter_mut() {
        sprite.1.scale = Vec3::ONE * camera_transform.scale();
    }
}
