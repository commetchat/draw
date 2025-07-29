use bevy::{
    color::ColorToComponents,
    ecs::event::{EventReader, EventWriter},
    log::info,
};

use crate::{
    BACKGROUND,
    active_strokes::active_stroke::RemoveStrokeEvent,
    database::{web_database::store_multiple_strokes, web_stroke_data::JsStrokeData},
    line_builder::LineBuilder,
    mesh_conversion::{stroke_to_mesh, timestamp_to_z_offset},
    networking::packet::ReceivedPacket,
    stroke::{Stroke, StrokeMesh},
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
