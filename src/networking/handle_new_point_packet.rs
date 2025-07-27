use bevy::ecs::event::{EventReader, EventWriter};

use crate::{active_strokes::active_stroke::ActiveStrokeEvent, networking::packet::ReceivedPacket};

pub fn handle_new_point_system(
    mut events: EventReader<ReceivedPacket>,
    mut point_events: EventWriter<ActiveStrokeEvent>,
) {
    for event in events.read() {
        match &event.data {
            super::packet::PacketData::NewPoint(new_point_packet_data) => {
                let mut data = new_point_packet_data.data.clone();
                data.owner = Some(event.from.clone());

                point_events.write(ActiveStrokeEvent::NewPoint(data));
            }
            _ => (),
        }
    }
}
