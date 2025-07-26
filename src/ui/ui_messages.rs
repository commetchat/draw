use bevy::ecs::event::Event;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Debug, Serialize, Deserialize, Default, Clone)]
#[ts(export, export_to = "../web/src/bindings/ui_binding.ts")]
pub struct PaintbrushArgs {
    pub width: f32,
    pub color: [f32; 3],
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[ts(export, export_to = "../web/src/bindings/ui_binding.ts")]
pub struct EraserArgs {
    width: f32,
}

#[derive(TS, Debug, Serialize, Deserialize)]
#[serde(tag = "tool")]
#[ts(export, export_to = "../web/src/bindings/ui_binding.ts")]
pub enum Tool {
    Paintbrush(PaintbrushArgs),
    Eraser(EraserArgs),
}

#[derive(TS, Debug, Event, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export, export_to = "../web/src/bindings/ui_binding.ts")]
pub enum UIMessage {
    LoadFile,
    SaveFile,
    SetTool(Tool),
    Undo,
    Redo,
}
