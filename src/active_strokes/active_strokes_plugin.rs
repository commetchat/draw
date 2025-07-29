use bevy::{
    app::{Plugin, PostUpdate, Update},
    ecs::{component::Component, schedule::IntoScheduleConfigs},
};

use crate::active_strokes::{
    active_stroke::{ActiveStrokeEvent, RemoveStrokeEvent},
    active_stroke_spawner::spawn_strokes_system,
    active_stroke_updater::{remove_strokes_system, update_strokes_system},
};

pub struct ActiveStrokesPlugin;

#[derive(Component, Default, Debug)]
pub struct ActiveTool {}

impl Plugin for ActiveStrokesPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<ActiveStrokeEvent>();
        app.add_event::<RemoveStrokeEvent>();
        app.add_systems(
            PostUpdate,
            (spawn_strokes_system, update_strokes_system).chain(),
        );
        app.add_systems(Update, remove_strokes_system);
    }
}
