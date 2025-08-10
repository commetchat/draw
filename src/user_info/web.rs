use std::sync::{LazyLock, Mutex};

use bevy::log::info;
use wasm_bindgen::prelude::wasm_bindgen;

pub static USER_ID: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("unset".to_string()));

#[wasm_bindgen]
pub fn web_set_user_id(user_id: String) {
    let mut id = USER_ID.lock().unwrap();
    info!("Setting our user id: {}", user_id);
    *id = user_id.clone();
}

pub fn web_get_user_id() -> String {
    let id = USER_ID.lock().unwrap();
    return id.clone();
}
