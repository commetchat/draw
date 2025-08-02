use bevy::{
    ecs::event::EventReader,
    log::info,
};

use crate::{
    active_strokes::active_stroke::RemoveStrokeEvent,
    networking::{
        Networking,
        packet::PacketData,
        packets::stroke_removed::StrokeRemovedPacket,
    },
};

pub fn send_removed_strokes_system(mut events: EventReader<RemoveStrokeEvent>) {
    for event in events.read() {
        // Dont send event which was sent to us
        if event.owner.is_some() {
            return;
        }

        let packet = PacketData::StrokeRemoved(StrokeRemovedPacket {
            timestamp: event.timestamp,
            id_random: event.id_random,
        });

        info!("Sending removed stroke packet!");

        for id in Networking::get_currently_connected_peers().iter() {
            Networking::send_to(id, &packet);
        }
    }
}
