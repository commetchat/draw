use bevy::{
    app::{Plugin, PostUpdate, Update},
    ecs::{component::Component, schedule::IntoScheduleConfigs},
};

use crate::active_strokes::{
    active_stroke::ActiveStrokeEvent, active_stroke_spawner::spawn_strokes_system,
    active_stroke_updater::update_strokes_system,
};

pub struct ActiveStrokesPlugin;

#[derive(Component, Default, Debug)]
pub struct ActiveTool {}

impl Plugin for ActiveStrokesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<ActiveStrokeEvent>();
        app.add_systems(
            PostUpdate,
            (spawn_strokes_system, update_strokes_system).chain(),
        );
    }
}
