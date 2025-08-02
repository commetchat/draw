use bevy::{
    ecs::event::{EventReader, EventWriter},
    log::info,
};

use crate::{
    active_strokes::active_stroke::RemoveStrokeEvent,
    load_file::{self, load_file},
    networking::packet::ReceivedPacket,
};

pub fn handle_received_save_file(
    mut packets: EventReader<ReceivedPacket>,
    mut writer: EventWriter<RemoveStrokeEvent>,
) {
    for event in packets.read() {
        match &event.data {
            super::packet::PacketData::SaveFileData(data) => {
                info!("Received save file!");

                info!("Len: {}", data.data.len());
                load_file(data.data.clone());
            }
            _ => (),
        }
    }
}
