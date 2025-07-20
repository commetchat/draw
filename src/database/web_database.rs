use wasm_bindgen::prelude::wasm_bindgen;

use crate::database::web_stroke_data::JsStrokeData;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn store_stroke(s: JsStrokeData);

    #[wasm_bindgen(js_namespace = gameDatabase, js_name=init)]
    pub fn init_web_database();

    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn store_multiple_strokes(s: Vec<JsStrokeData>);

    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn load_mesh_for_chunk(id: String);
}
