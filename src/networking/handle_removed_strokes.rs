use bevy::{
    ecs::event::{EventReader, EventWriter},
    log::info,
};

use crate::{
    active_strokes::active_stroke::RemoveStrokeEvent,
    networking::packet::ReceivedPacket,
};

pub fn handle_removed_strokes(
    mut packets: EventReader<ReceivedPacket>,
    mut writer: EventWriter<RemoveStrokeEvent>,
) {
    for event in packets.read() {
        match &event.data {
            super::packet::PacketData::StrokeRemoved(stroke_removed_packet) => {
                info!("Handling removed stroke packet");
                writer.write(RemoveStrokeEvent {
                    timestamp: stroke_removed_packet.timestamp,
                    id_random: stroke_removed_packet.id_random,
                    owner: Some(event.from.clone()),
                });
            }
            _ => (),
        }
    }
}
