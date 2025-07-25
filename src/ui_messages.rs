use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "../web/src/bindings/ui_binding.ts")]
pub struct PaintbrushArgs {
    width: f32,
    color: [f32; 3],
}

#[derive(TS, Serialize, Deserialize)]
#[ts(export, export_to = "../web/src/bindings/ui_binding.ts")]
pub struct EraserArgs {
    width: f32,
}

#[derive(TS, Serialize, Deserialize)]
#[serde(tag = "tool")]
#[ts(export, export_to = "../web/src/bindings/ui_binding.ts")]
pub enum Tool {
    Paintbrush(PaintbrushArgs),
    Eraser(EraserArgs),
}

#[derive(TS, Serialize, Deserialize)]
#[serde(tag = "type")]
#[ts(export, export_to = "../web/src/bindings/ui_binding.ts")]
pub enum UIMessage {
    LoadFile,
    SaveFile,
    SetTool(Tool),
    Undo,
    Redo,
}
