use wasm_bindgen::prelude::wasm_bindgen;

use crate::database::web_stroke_data::JsStrokeData;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn store_stroke(s: JsStrokeData);

    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn store_multiple_strokes(s: Vec<JsStrokeData>);

    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn append_chunk_data(
        id: String,
        vertices: Vec<u8>,
        indices: Vec<u32>,
        colors: Vec<u8>,
        strokes: Vec<JsStrokeData>,
    );

    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn load_mesh_for_chunk(id: String);

    #[wasm_bindgen(js_namespace = gameDatabase)]
    pub fn delete_stroke(id: String);

}
