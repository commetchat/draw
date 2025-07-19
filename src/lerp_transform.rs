//! Displays touch presses, releases, and cancels.

use bevy::prelude::*;

pub struct LerpTransformPlugin;

#[derive(Component, Default)]
pub struct TargetTransform {
    pub transform: Transform,
}

impl Plugin for LerpTransformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, lerp);
    }
}

fn lerp(mut query: Query<(&mut Transform, &TargetTransform)>, time: Res<Time>) {
    for (mut transform, target) in query.iter_mut() {
        let lerp_amount = (time.delta_secs() * 20.0).clamp(0.0, 1.0);

        transform.translation = transform
            .translation
            .lerp(target.transform.translation, lerp_amount);

        transform.rotation = transform
            .rotation
            .lerp(target.transform.rotation, lerp_amount);

        transform.scale = transform.scale.lerp(target.transform.scale, lerp_amount);
    }
}
