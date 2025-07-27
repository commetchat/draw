use std::sync::{LazyLock, Mutex};

use bevy::log::info;
use wasm_bindgen::prelude::wasm_bindgen;

pub static CONNECTED_PEERS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));

#[wasm_bindgen]
pub fn web_peer_connected(peer_id: String) {
    let mut queue = CONNECTED_PEERS.lock().unwrap();
    if queue.iter().any(|f| f == &peer_id) {
        return;
    }

    queue.push(peer_id.clone());

    info!("Currently connected peers: {:?}", queue);
}

#[wasm_bindgen]
pub fn web_peer_disconnected(peer_id: String) {
    let mut queue = CONNECTED_PEERS.lock().unwrap();

    queue.retain(|f| f != &peer_id);
}

pub fn get_currently_connected_peers() -> Vec<String> {
    let queue = CONNECTED_PEERS.lock().unwrap();
    queue.clone()
}
