use bevy::{log::info, math::Vec2};
use binary_util::{ByteReader, ByteWriter};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    chunks::position_to_chunk_id,
    stroke::{Stroke, StrokeData, StrokeMesh, StrokeMetadata, StrokeType},
};

#[wasm_bindgen(getter_with_clone, js_name = "StrokeData")]
pub struct JsStrokeData {
    pub id: String,
    pub id_random: u32,
    pub timestamp: f64,
    pub chunk_key: String,
    pub origin_x: f32,
    pub origin_y: f32,
    pub owner_id: Option<String>,
    pub stroke_data: Vec<u8>,
    pub vertex_data: Vec<u8>,
    pub index_data: Vec<u32>,
    pub color_data: Vec<u8>,
}

#[wasm_bindgen(getter_with_clone, js_name = "MeshData")]
pub struct JsMeshData {
    pub chunk_key: String,
    pub vertex_data: Vec<u8>,
    pub index_data: Vec<u32>,
    pub color_data: Vec<u8>,
}

#[wasm_bindgen(js_class = "MeshData")]
impl JsMeshData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        chunk_key: String,
        vertex_data: Vec<u8>,
        index_data: Vec<u32>,
        color_data: Vec<u8>,
    ) -> JsMeshData {
        JsMeshData {
            chunk_key: chunk_key,
            vertex_data: vertex_data,
            index_data: index_data,
            color_data: color_data,
        }
    }
}

#[wasm_bindgen(js_class = "StrokeData")]
impl JsStrokeData {
    #[wasm_bindgen(constructor)]
    pub fn new(
        id: String,
        id_random: u32,
        chunk_key: String,
        timestamp: f64,
        origin_x: f32,
        origin_y: f32,
        owner_id: Option<String>,
        stroke_data: Vec<u8>,
        vertex_data: Vec<u8>,
        index_data: Vec<u32>,
        color_data: Vec<u8>,
    ) -> JsStrokeData {
        JsStrokeData {
            id: id,
            id_random: id_random,
            chunk_key: chunk_key,
            origin_x: origin_x,
            origin_y: origin_y,
            timestamp: timestamp,
            owner_id: owner_id,
            stroke_data: stroke_data,
            vertex_data: vertex_data,
            index_data: index_data,
            color_data: color_data,
        }
    }
}

impl JsStrokeData {
    pub fn from_stroke(stroke: &Stroke) -> JsStrokeData {
        let mut writer = ByteWriter::new();

        let binary_data = stroke.data.write_data();

        let chunk_key = position_to_chunk_id(stroke.metadata.origin);

        return JsStrokeData {
            id: stroke.metadata.get_id(),
            id_random: stroke.metadata.id_random,
            timestamp: stroke.metadata.timestamp,
            chunk_key: chunk_key,
            origin_x: stroke.metadata.origin.x,
            origin_y: stroke.metadata.origin.y,
            owner_id: stroke.metadata.owner.clone(),
            stroke_data: binary_data,
            vertex_data: stroke.mesh.write_vertex_data(),
            index_data: stroke.mesh.indices.clone(),
            color_data: stroke.mesh.write_color_data(),
        };
    }
}
