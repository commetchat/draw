use bevy::{
    app::{App, Plugin, Update},
    ecs::event::EventReader,
};

use crate::{
    database::{
        web_database::{store_multiple_strokes, store_stroke},
        web_stroke_data::JsStrokeData,
    },
    stroke::StrokeEvent,
};

mod web_database;
mod web_stroke_data;

pub struct Database;

impl Plugin for Database {
    fn build(&self, app: &mut App) {
        #[cfg(target_arch = "wasm32")]
        {
            app.add_systems(Update, store_finished_strokes);
        }
    }
}

fn store_finished_strokes(mut events: EventReader<StrokeEvent>) {
    let mut data = Vec::new();

    for event in events.read() {
        match event {
            StrokeEvent::StrokeFinished(stroke) => {
                data.push(JsStrokeData {
                    id: stroke.metadata.get_id(),
                    id_random: stroke.metadata.id_random,
                    timestamp: stroke.metadata.timestamp,
                    owner_id: stroke.metadata.owner.clone(),
                    chunk_key: "0_0".to_string(),
                    stroke_data: Vec::new(),
                });

                // Pass to database in batches, to slightly reduce memory pressure
                if data.len() > 500 {
                    store_multiple_strokes(data);
                    data = Vec::new();
                }
            }
            _ => (),
        }
    }

    if !data.is_empty() {
        store_multiple_strokes(data);
    }
}
