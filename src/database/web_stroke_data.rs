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
    pub mesh_data: Vec<u8>,
}

use bincode::{Decode, Encode};

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
        mesh_data: Vec<u8>,
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
            mesh_data: mesh_data,
        }
    }
}

impl JsStrokeData {
    pub fn from_stroke(stroke: &Stroke) -> JsStrokeData {
        let mut writer = ByteWriter::new();

        let binary_data = stroke.data.write_data();
        let mesh_data = stroke.mesh.write_data();

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
            mesh_data: mesh_data,
        };
    }

    pub fn to_stroke(&self) -> Stroke {
        Stroke {
            metadata: StrokeMetadata {
                timestamp: self.timestamp,
                id_random: self.id_random,
                owner: self.owner_id.clone(),
                origin: Vec2 {
                    x: self.origin_x,
                    y: self.origin_y,
                },
            },
            data: StrokeData::from_bytes(self.stroke_data.clone()),
            mesh: StrokeMesh::from_bytes(self.mesh_data.clone()),
        }
    }
}
