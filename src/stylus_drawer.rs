use bevy::{color::palettes::css::RED, prelude::*};

use crate::{retained_view::copy_camera::TargetCamera, stylus_input::StylusEvent};

pub struct StylusDrawer;

impl Plugin for StylusDrawer {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            show_gizmo.after(TransformSystem::TransformPropagate),
        );
    }
}

fn show_gizmo(
    mut events: EventReader<StylusEvent>,
    camera_query: Single<(&Camera, &GlobalTransform), With<TargetCamera>>,
    mut window: Single<&mut Window>,
    mut gizmos: Gizmos,
) {
    let (camera, camera_transform) = *camera_query;
    let scale = window.resolution.base_scale_factor();

    for event in events.read() {
        match event {
            StylusEvent::PointerMove(pointer_data) => {
                if let Ok(world_pos) =
                    camera.viewport_to_world_2d(camera_transform, pointer_data.position * scale)
                {
                    gizmos.circle_2d(world_pos, 10.0 + pointer_data.pressure * 50.0, RED);
                }
            }
        }
    }
}
