use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        system::{Commands, Query, Res},
    },
    log::info,
    time::Time,
};

#[derive(Debug, Component)]
pub struct ActiveStrokeLifetime {
    pub remaining_life: f64,
}

pub fn stroke_lifetime_system(
    mut current_strokes: Query<(&mut ActiveStrokeLifetime, Entity)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    for mut stroke in current_strokes.iter_mut() {
        stroke.0.remaining_life -= time.delta_secs_f64();

        if stroke.0.remaining_life < 0.0 {
            commands.entity(stroke.1).despawn();
            info!("Active stroke lifetime ran out, despawning!");
        }
    }
}
