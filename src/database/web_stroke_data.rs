use wasm_bindgen::prelude::wasm_bindgen;

use crate::stroke::StrokeType;

#[wasm_bindgen(getter_with_clone, js_name = "StrokeData")]
pub struct JsStrokeData {
    pub id: String,
    pub id_random: u32,
    pub timestamp: f64,
    pub chunk_key: String,
    pub owner_id: Option<String>,
    pub stroke_data: Vec<u8>,
}

#[wasm_bindgen(js_class = "StrokeData")]
impl JsStrokeData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: String,
        id_random: u32,
        chunk_key: String,
        timestamp: f64,
        owner_id: Option<String>,
        stroke_data: Vec<u8>,
    ) -> JsStrokeData {
        JsStrokeData {
            id: id,
            id_random: id_random,
            chunk_key: chunk_key,
            timestamp: timestamp,
            owner_id: owner_id,
            stroke_data: stroke_data,
        }
    }
}
