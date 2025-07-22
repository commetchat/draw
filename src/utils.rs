use std::time::{SystemTime, UNIX_EPOCH};

use wasm_bindgen::prelude::wasm_bindgen;

pub const DEBUG_DRAW: bool = false;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance, js_name = "now")]
    pub fn web_now() -> f64;
}

pub fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    return web_now();

    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    t.as_secs_f64() * 0.001
}
