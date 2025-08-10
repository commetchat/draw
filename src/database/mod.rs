use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

use bevy::{
    app::{App, Plugin},
    ecs::event::EventReader,
    log::info,
    platform::collections::HashMap,
};

#[cfg(target_arch = "wasm32")]
use bevy::app::Update;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    database::{
        web_database::store_multiple_strokes,
        web_stroke_data::{JsMeshData, JsStrokeData},
    },
    stroke::StrokeEvent,
    ui::{ui_messages::UIMessage::GameReady, ui_messages_queue::send_ui_message},
};

pub mod web_database;
pub mod web_stroke_data;

pub struct Database;

pub struct RemoveVertsData {
    pub chunk_key: String,
    pub offset: u32,
    pub num_verts: u32,
}

pub static LOAD_MESH_QUEUE: LazyLock<Mutex<HashMap<String, JsMeshData>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub static CHUNKS_NEED_RELOADING: LazyLock<Mutex<Vec<String>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub static APPEND_STROKE_DATAS: LazyLock<Mutex<HashMap<String, VecDeque<JsStrokeData>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

pub static REMOVE_CHUNK_VERTS: LazyLock<Mutex<VecDeque<RemoveVertsData>>> =
    LazyLock::new(|| Mutex::new(VecDeque::new()));

pub static DATABASE_READY: Mutex<bool> = Mutex::new(false);

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
                info!("Storing stroke by: {:?}", stroke.metadata.owner);
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
pub fn db_append_mesh_data(strokes: Vec<JsStrokeData>) {
    let mut map = APPEND_STROKE_DATAS.lock().unwrap();

    for stroke in strokes.iter() {
        if map.contains_key(&stroke.chunk_key) == false {
            map.insert(stroke.chunk_key.clone(), VecDeque::new());
        }

        let queue = map.get_mut(&stroke.chunk_key).unwrap();

        queue.push_back(stroke.clone());
    }
}

#[wasm_bindgen]
pub fn db_on_mesh_loaded(mesh: JsMeshData) {
    let mut map = LOAD_MESH_QUEUE.lock().unwrap();

    map.insert(mesh.chunk_key.clone(), mesh);
}

#[wasm_bindgen]
pub fn db_chunk_needs_reloading(chunk: String) {
    info!("Chunk needs reloading: {}", chunk);
    let mut list = CHUNKS_NEED_RELOADING.lock().unwrap();
    list.push(chunk);
}

#[wasm_bindgen]
pub fn db_remove_verts(chunk_key: String, offset: u32, num_verts: u32) {
    info!("Chunk needs verts removed: {}", chunk_key);
    let mut list = REMOVE_CHUNK_VERTS.lock().unwrap();
    list.push_back(RemoveVertsData {
        chunk_key: chunk_key,
        offset: offset,
        num_verts: num_verts,
    });
}

#[wasm_bindgen]
pub fn db_ready() {
    info!("Received database ready signal");

    let mut ready = DATABASE_READY.lock().unwrap();
    *ready = true;

    send_ui_message(GameReady);
}
