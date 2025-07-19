//! Displays touch presses, releases, and cancels.

use bevy::{input::touch::*, prelude::*};

use crate::lerp_transform::TargetTransform;

pub struct CameraControllerPlugin;

#[derive(Component, Default)]
pub struct TouchCameraController {
    last_position_a: Option<Vec2>,
    last_position_b: Option<Vec2>,
}

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, touch_system);
    }
}

fn touch_system(
    touches: Res<Touches>,
    mut camera_query: Single<(
        &mut TargetTransform,
        &mut TouchCameraController,
        &Camera,
        &GlobalTransform,
    )>,
) {
    let t: Vec<&Touch> = touches.iter().collect();

    let mut a: Option<&Touch> = None;
    let mut b: Option<&Touch> = None;

    let prev_a = camera_query.1.last_position_a;
    let prev_b = camera_query.1.last_position_b;

    let mut new_transform = camera_query.0.transform;

    if t.len() > 0 {
        a = Some(*t.get(0).unwrap());
    }

    if t.len() > 1 {
        b = Some(*t.get(1).unwrap());
    }

    if prev_b.is_some() && b.is_none() {
        a = None;
        b = None;
    }

    if a.is_some() && b.is_none() && prev_a.is_some() {
        info!("A: {}", a.unwrap().id());
        if let Ok(initial_focal_point) = camera_query
            .2
            .viewport_to_world_2d(camera_query.3, prev_a.unwrap())
        {
            if let Ok(current_focal_point) = camera_query
                .2
                .viewport_to_world_2d(camera_query.3, a.unwrap().position())
            {
                let diff = initial_focal_point - current_focal_point;
                new_transform.translation.x += diff.x;
                new_transform.translation.y += diff.y;
            }
        }
    }

    if a.is_some() && b.is_some() && prev_a.is_some() && prev_b.is_some() {
        let initial_distance = prev_a.unwrap().distance(prev_b.unwrap());
        let initial_angle = (prev_a.unwrap() - prev_b.unwrap()).to_angle();

        let new_distance = a.unwrap().position().distance(b.unwrap().position());
        let new_angle = (a.unwrap().position() - b.unwrap().position()).to_angle();

        let diff = new_angle - initial_angle;

        new_transform.scale *= initial_distance / new_distance;
        new_transform.rotate_local_z(diff);
    }

    camera_query.1.last_position_a = match a {
        Some(a) => Some(a.position()),
        None => None,
    };

    camera_query.1.last_position_b = match b {
        Some(b) => Some(b.position()),
        None => None,
    };

    camera_query.0.transform = new_transform;
}
