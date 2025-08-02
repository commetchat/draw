use bevy::ecs::{event::EventReader, system::Query};

use crate::{
    active_strokes::active_stroke::{ActiveStroke, ActiveStrokeEvent},
    networking::{
        Networking,
        packet::PacketData,
        packets::{new_point::NewPointPacketData, stroke_complete::StrokeCompleteData},
    },
    player_sprite::get_player_state::PlayerStateData,
    stroke::{Stroke, StrokeData, StrokeMetadata},
};

pub fn send_player_state_system(mut events: EventReader<PlayerStateData>) {
    for event in events.read() {
        let packet = PacketData::PlayerState(event.clone());

        for id in Networking::get_currently_connected_peers().iter() {
            Networking::send_to(id, &packet);
        }
    }
}
