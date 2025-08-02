use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

use bevy::{
    ecs::event::{Event, EventWriter},
    log::info,
};
use wasm_bindgen::prelude::wasm_bindgen;

pub static CONNECTED_PEERS: LazyLock<Mutex<Vec<String>>> = LazyLock::new(|| Mutex::new(Vec::new()));
pub static JUST_CONNECTED_PEERS: LazyLock<Mutex<VecDeque<String>>> =
    LazyLock::new(|| Mutex::new(VecDeque::new()));

#[wasm_bindgen]
pub fn web_peer_connected(peer_id: String) {
    let mut connected_peers = CONNECTED_PEERS.lock().unwrap();
    if connected_peers.iter().any(|f| f == &peer_id) {
        return;
    }

    let mut just_connected = JUST_CONNECTED_PEERS.lock().unwrap();
    just_connected.push_back(peer_id.clone());
    connected_peers.push(peer_id.clone());

    info!("Currently connected peers: {:?}", connected_peers);
}

#[wasm_bindgen]
pub fn web_peer_disconnected(peer_id: String) {
    let mut connected_peers = CONNECTED_PEERS.lock().unwrap();

    connected_peers.retain(|f| f != &peer_id);
}

pub fn get_currently_connected_peers() -> Vec<String> {
    let connected_peers = CONNECTED_PEERS.lock().unwrap();
    connected_peers.clone()
}

#[derive(Event)]
pub struct PeerConnectedEvent {
    pub id: String,
}

pub fn connected_peers_event_system(mut writer: EventWriter<PeerConnectedEvent>) {
    let mut queue = JUST_CONNECTED_PEERS.lock().unwrap();

    while let Some(item) = queue.pop_front() {
        writer.write(PeerConnectedEvent { id: item.clone() });
    }
}
