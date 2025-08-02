use bevy::ecs::event::EventReader;

use crate::{
    database::{web_database::store_multiple_strokes, web_stroke_data::JsStrokeData},
    mesh_conversion::stroke_to_mesh,
    networking::packet::ReceivedPacket,
    stroke::StrokeMesh,
};

pub fn handle_received_strokes_system(mut events: EventReader<ReceivedPacket>) {
    for event in events.read() {
        match &event.data {
            super::packet::PacketData::StrokeComplete(stroke) => {
                let mut s = stroke.clone();

                let (verts, colors, indices) = stroke_to_mesh(&s.data);

                s.data.mesh = Some(StrokeMesh {
                    vertices: verts,
                    colors: colors,
                    indices: indices,
                });

                let mut data = JsStrokeData::from_stroke(&s.data);
                data.owner_id = Some(event.from.clone());

                store_multiple_strokes(vec![data]);
            }
            _ => (),
        }
    }
}
