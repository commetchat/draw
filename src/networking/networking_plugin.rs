use bevy::{
    app::{Plugin, PostUpdate, Update},
    ecs::schedule::IntoScheduleConfigs,
};

use crate::networking::{
    handle_new_point_packet::handle_new_point_system,
    handle_received_strokes::handle_received_strokes_system,
    handle_removed_strokes::handle_removed_strokes, packet::ReceivedPacket,
    receive_packet::parse_packet_system, send_active_strokes::send_active_strokes_system,
    send_removed_strokes::send_removed_strokes_system,
};

pub struct NetworkingPlugin;

impl Plugin for NetworkingPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<ReceivedPacket>();
        app.add_systems(
            PostUpdate,
            (parse_packet_system, handle_received_strokes_system).chain(),
        );
        app.add_systems(Update, send_active_strokes_system);
        app.add_systems(Update, handle_new_point_system);
        app.add_systems(Update, send_removed_strokes_system);
        app.add_systems(Update, handle_removed_strokes);
    }
}
