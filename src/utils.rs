use std::time::{SystemTime, UNIX_EPOCH};

#[cfg(not(target_arch = "wasm32"))]
use rand::RngCore;

use wasm_bindgen::prelude::wasm_bindgen;

pub const DEBUG_DRAW: bool = false;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = performance, js_name = "now")]
    pub fn web_now() -> f64;
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = gameUtils, js_name = "get_system_time")]
    pub fn web_get_system_time() -> f64;

    #[wasm_bindgen(js_namespace = gameUtils, js_name = "get_random_u32")]
    pub fn web_get_random_uint32() -> u32;
}

// Returns an arbitrary time, in milliseconds
// On desktop, this is system time, on web, performance.now
pub fn now() -> f64 {
    #[cfg(target_arch = "wasm32")]
    return web_now();

    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    t.as_secs_f64() * 0.001
}

pub fn get_system_time() -> f64 {
    #[cfg(target_arch = "wasm32")]
    return web_get_system_time();

    let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    return t.as_secs_f64();
}

pub fn get_random_uint32() -> u32 {
    #[cfg(target_arch = "wasm32")]
    return web_get_random_uint32();

    #[cfg(not(target_arch = "wasm32"))]
    {
        let mut rng = rand::rng();
        return rng.next_u32();
    }
}
