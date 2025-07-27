use bevy::{color::ColorToComponents, ecs::event::EventReader, log::info};

use crate::{
    BACKGROUND,
    database::{web_database::store_multiple_strokes, web_stroke_data::JsStrokeData},
    line_builder::LineBuilder,
    mesh_conversion::{stroke_to_mesh, timestamp_to_z_offset},
    networking::packet::ReceivedPacket,
    stroke::{Stroke, StrokeMesh},
};

pub fn store_received_strokes_system(mut events: EventReader<ReceivedPacket>) {
    for event in events.read() {
        match &event.data {
            super::packet::PacketData::StrokeComplete(stroke) => {
                let mut s = stroke.clone();

                let (verts, colors, indices) = stroke_to_mesh(&s);

                s.mesh = Some(StrokeMesh {
                    vertices: verts,
                    colors: colors,
                    indices: indices,
                });

                let mut data = JsStrokeData::from_stroke(&s);
                data.owner_id = Some(event.from.clone());

                store_multiple_strokes(vec![data]);
            }
            _ => (),
        }
    }
}
