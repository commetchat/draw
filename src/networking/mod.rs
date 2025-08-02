use bevy::log::info;
use binary_util::ByteWriter;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::networking::{
    connected_peers::get_currently_connected_peers,
    packet::{PacketData, write_packet},
};

pub mod connected_peers;
mod handle_new_point_packet;
mod handle_player_state;
mod handle_received_strokes;
mod handle_removed_strokes;
pub mod network_owned;
pub mod networking_plugin;
pub mod packet;
pub mod packets;
mod receive_packet;
mod send_active_strokes;
mod send_player_state;
mod send_removed_strokes;

pub struct Networking;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = gameUtils)]
    pub fn web_send_packet(to: String, data: Vec<u8>);
}

impl Networking {
    pub fn get_currently_connected_peers() -> Vec<String> {
        return get_currently_connected_peers();
    }

    pub fn send_to(id: &String, data: &PacketData) {
        let mut writer = ByteWriter::new();
        match write_packet(&mut writer, data) {
            Ok(_) => {
                web_send_packet(id.clone(), writer.as_slice().to_vec());
            }
            Err(_) => {
                info!("Failed to send packet!");
            }
        }
    }
}
