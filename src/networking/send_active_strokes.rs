use bevy::ecs::{event::EventReader, system::Query};

use crate::{
    active_strokes::active_stroke::{ActiveStroke, ActiveStrokeEvent},
    networking::{
        Networking,
        packet::PacketData,
        packets::{new_point::NewPointPacketData, stroke_complete::StrokeCompleteData},
    },
    stroke::{Stroke, StrokeData, StrokeMetadata},
};

pub fn send_active_strokes_system(
    mut events: EventReader<ActiveStrokeEvent>,
    mut current_strokes: Query<&ActiveStroke>,
) {
    for event in events.read() {
        match event {
            ActiveStrokeEvent::NewPoint(new_point_data) => {
                // Dont send events which were sent to us!
                if new_point_data.owner.is_some() {
                    continue;
                }

                let packet = PacketData::NewPoint(NewPointPacketData {
                    data: new_point_data.clone(),
                });

                Networking::broadcast(&packet);
            }
            ActiveStrokeEvent::StrokeFinished(stroke_finished_data) => {
                for stroke in current_strokes.iter_mut() {
                    if stroke.timestamp == stroke_finished_data.timestamp
                        && stroke.id_random == stroke_finished_data.id_random
                    {
                        let data = Stroke {
                            metadata: StrokeMetadata {
                                id_random: stroke.id_random,
                                timestamp: stroke.timestamp,
                                owner: None,
                                origin: stroke.stroke_origin,
                                source: stroke_finished_data.source.clone(),
                            },
                            data: StrokeData {
                                stroke_type: crate::stroke::StrokeType::Paint(stroke.color),
                                width: stroke.width,
                                points: stroke.points.clone(),
                                pressures: Some(stroke.pressures.clone()),
                            },
                            mesh: None,
                        };

                        let packet = PacketData::StrokeComplete(StrokeCompleteData { data: data });

                        Networking::broadcast(&packet);
                    }
                }
            }

            _ => (),
        }
    }
}
