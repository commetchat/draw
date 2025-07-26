use bevy::{log::info, math::Vec2};
use binary_util::{ByteReader, ByteWriter};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    chunks::position_to_chunk_id,
    stroke::{Stroke, StrokeData, StrokeMesh, StrokeMetadata, StrokeType},
};

#[wasm_bindgen(getter_with_clone, js_name = "StrokeData")]
#[derive(Clone)]
pub struct JsStrokeData {
    pub id: String,
    pub id_random: u32,
    pub timestamp: f64,
    pub chunk_key: String,
    pub origin_x: f32,
    pub origin_y: f32,
    pub owner_id: Option<String>,
    pub vertex_offset: Option<u32>,
    pub num_verts: Option<u32>,
    pub stroke_data: Vec<u8>,
    pub vertex_data: Option<Vec<u8>>,
    pub index_data: Option<Vec<u32>>,
    pub color_data: Option<Vec<u8>>,
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
        vertex_offset: Option<u32>,
        num_verts: Option<u32>,
        vertex_data: Option<Vec<u8>>,
        index_data: Option<Vec<u32>>,
        color_data: Option<Vec<u8>>,
    ) -> JsStrokeData {
        JsStrokeData {
            id: id,
            id_random: id_random,
            chunk_key: chunk_key,
            origin_x: origin_x,
            origin_y: origin_y,
            timestamp: timestamp,
            owner_id: owner_id,
            vertex_offset: vertex_offset,
            num_verts: num_verts,
            stroke_data: stroke_data,
            vertex_data: vertex_data,
            index_data: index_data,
            color_data: color_data,
        }
    }
}

impl JsStrokeData {
    pub fn from_stroke(stroke: &Stroke) -> JsStrokeData {
        let binary_data = stroke.data.write_data();

        let chunk_key = position_to_chunk_id(stroke.metadata.origin);

        let mesh_data = match &stroke.mesh {
            Some(mesh) => (
                Some(mesh.write_vertex_data()),
                Some(mesh.indices.clone()),
                Some(mesh.write_color_data()),
            ),
            None => (None, None, None),
        };

        let num_verts = match &stroke.mesh {
            Some(mesh) => Some(u32::try_from(mesh.vertices.len()).unwrap()),
            None => None,
        };

        return JsStrokeData {
            id: stroke.metadata.get_id(),
            id_random: stroke.metadata.id_random,
            timestamp: stroke.metadata.timestamp,
            chunk_key: chunk_key,
            origin_x: stroke.metadata.origin.x,
            origin_y: stroke.metadata.origin.y,
            owner_id: stroke.metadata.owner.clone(),
            stroke_data: binary_data,
            num_verts: num_verts,
            vertex_offset: None,
            vertex_data: mesh_data.0,
            index_data: mesh_data.1,
            color_data: mesh_data.2,
        };
    }
}
