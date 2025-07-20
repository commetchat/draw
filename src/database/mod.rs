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
        web_database::{store_multiple_strokes, store_stroke},
        web_stroke_data::{JsMeshData, JsStrokeData},
    },
    line_builder::{LineBuilder, LineCapMode, LineJointMode},
    stroke::{Stroke, StrokeEvent},
    utils::now,
};

pub mod web_database;
pub mod web_stroke_data;

pub struct Database;

pub static LOAD_MESH_QUEUE: LazyLock<Mutex<HashMap<String, VecDeque<JsMeshData>>>> =
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
pub fn db_on_mesh_loaded(mesh: JsMeshData) {
    info!("Received mesh from db");

    let mut map = LOAD_MESH_QUEUE.lock().unwrap();

    if map.contains_key(&mesh.chunk_key) == false {
        map.insert(mesh.chunk_key.clone(), VecDeque::new());
    }

    let queue = map.get_mut(&mesh.chunk_key).unwrap();

    queue.push_back(mesh);
}
