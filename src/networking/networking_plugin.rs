use bevy::{
    app::{Plugin, PostUpdate},
    ecs::schedule::IntoScheduleConfigs,
};

use crate::networking::{
    packet::ReceivedPacket, receive_packet::parse_packet_system,
    store_received_strokes::store_received_strokes_system,
};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<ReceivedPacket>();
        app.add_systems(
            PostUpdate,
            (parse_packet_system, store_received_strokes_system).chain(),
        );
    }
}
