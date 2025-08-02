use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

use bevy::{ecs::event::EventWriter, log::info};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::networking::packet::{ReceivedPacket, parse_packet};

pub static PACKET_QUEUE: LazyLock<Mutex<VecDeque<(String, Vec<u8>)>>> =
    LazyLock::new(|| Mutex::new(VecDeque::new()));

#[wasm_bindgen]
pub fn web_receive_packet(from: String, data: Vec<u8>) {
    let mut queue = PACKET_QUEUE.lock().unwrap();
    queue.push_back((from, data));
}

pub fn parse_packet_system(mut events: EventWriter<ReceivedPacket>) {
    let mut queue = PACKET_QUEUE.lock().unwrap();

    while let Some(item) = queue.pop_front() {
        let packet_data = parse_packet(item.1);

        match packet_data {
            Ok(data) => {
                events.write(ReceivedPacket {
                    from: item.0.clone(),
                    data: data,
                });
            }
            Err(err) => {
                info!("Failed to parse packet: {:?}", err);
                continue;
            }
        }
    }
}
