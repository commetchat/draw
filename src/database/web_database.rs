use wasm_bindgen::prelude::wasm_bindgen;

use crate::database::web_stroke_data::JsStrokeData;

#[wasm_bindgen]
pub fn db_on_stroke_loaded(stroke: JsStrokeData) {}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn store_stroke(s: JsStrokeData);

    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn store_multiple_strokes(s: Vec<JsStrokeData>);
}
