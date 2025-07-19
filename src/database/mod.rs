use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
    time::{SystemTime, UNIX_EPOCH},
};

use bevy::{
    app::{App, Plugin, Update},
    asset::{Assets, RenderAssetUsages},
    color::{Color, ColorToComponents, palettes::css::RED},
    ecs::{
        event::EventReader,
        system::{Commands, ResMut},
    },
    log::info,
    platform::collections::HashMap,
    render::mesh::{self, Mesh, Mesh2d},
    sprite::MeshMaterial2d,
    transform::components::Transform,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    CustomMaterial,
    chunks::{ChunkEvent, position_to_chunk_id},
    database::{
        web_database::{load_strokes_for_chunk, store_multiple_strokes, store_stroke},
        web_stroke_data::JsStrokeData,
    },
    line_builder::{LineBuilder, LineCapMode, LineJointMode},
    stroke::{Stroke, StrokeEvent},
    utils::now,
};

pub mod web_database;
pub mod web_stroke_data;

pub struct Database;

pub static LOAD_STROKE_QUEUE: LazyLock<Mutex<HashMap<String, VecDeque<Stroke>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

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
                data.push(JsStrokeData::from_stroke(stroke));

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

#[wasm_bindgen]
pub fn db_on_strokes_loaded(strokes: Vec<JsStrokeData>, chunk_id: String) {
    info!("Received {} strokes from db", strokes.len());

    let mut map = LOAD_STROKE_QUEUE.lock().unwrap();

    if map.contains_key(&chunk_id) == false {
        map.insert(chunk_id.clone(), VecDeque::new());
    }

    let queue = map.get_mut(&chunk_id).unwrap();

    for data in strokes.iter() {
        let stroke = data.to_stroke();
        queue.push_back(stroke);
    }
}
