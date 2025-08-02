use bevy::{
    app::{Plugin, Startup, Update},
    ecs::{component::Component, system::Commands},
    utils::default,
};

use crate::{
    tools::{
        color_picker::{ToolColorPicker, color_picker_system, color_picker_ui_system},
        paintbrush::{
            ToolPaintBrush, paintbrush_gizmo_system, paintbrush_system, paintbrush_ui_system,
        },
        undo::{store_undo_strokes_system, undo_ui_system},
    },
    ui::ui_messages::PaintbrushArgs,
};

pub struct ToolsPlugin;

#[derive(Component, Default, Debug)]
pub struct ActiveTool {}

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, paintbrush_ui_system);
        app.add_systems(Update, paintbrush_system);
        app.add_systems(Update, paintbrush_gizmo_system);
        app.add_systems(Update, color_picker_system);

        app.add_systems(Update, color_picker_ui_system);
        app.add_systems(Update, store_undo_strokes_system);
        app.add_systems(Update, undo_ui_system);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        ToolPaintBrush {
            args: PaintbrushArgs {
                width: 10.0,
                color: [1.0, 0.0, 0.0],
                ..default()
            },
            ..default()
        },
        ActiveTool {},
    ));

    commands.spawn(ToolColorPicker {});
}
