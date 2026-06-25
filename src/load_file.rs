use bevy::{log::info, math::Vec2};
use binary_util::ByteReader;
use safe_transmute::transmute_to_bytes;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    database::{web_database::append_chunk_data, web_stroke_data::{JSStrokeSource, JsStrokeData}}, mesh_conversion::stroke_to_mesh, stroke::{Stroke, StrokeData, StrokeMetadata, StrokeSource},
};

pub struct LoadFile;

#[wasm_bindgen]
pub fn load_file(bytes: Vec<u8>) {
    match load(bytes, JSStrokeSource::User, None) {
        Ok(_) => (),
        Err(_) => {
            info!("Failed to load from file!");
        }
    }
}

#[wasm_bindgen]
pub fn load_chunk(bytes: Vec<u8>, owner: String) {
    info!("Loading chunk from: {}", owner);

    match load(bytes, JSStrokeSource::Storage, Some(owner)) {
        Ok(_) => (),
        Err(_) => {
            info!("Failed to load from file!");
        }
    }
}

struct ChunkLoadResult {
    chunk_id: String,
    indices: Vec<u32>,
    colors: Vec<[f32; 4]>,
    vertices: Vec<[f32; 3]>,
    strokes: Vec<JsStrokeData>,
}

fn load(bytes: Vec<u8>, source: JSStrokeSource, user_id: Option<String>) -> Result<(), std::io::Error> {
    let mut reader = ByteReader::from(bytes);
    info!("Reading save file");

    let magic = reader.read_u32()?;
    let version = reader.read_u32()?;
    info!("Magic: {}", magic);
    info!("File version: {}", version);

    let mut found_user_ids = Vec::new();

    while reader.peek_ahead(1).is_ok() {
        let mut result = read_chunk(&mut reader, &mut found_user_ids)?;

        let vertices = transmute_to_bytes(&result.vertices);
        let colors = transmute_to_bytes(&result.colors);

        for stroke in &mut result.strokes {
            stroke.source = source.clone();

            if let Some(owner) = &user_id {
                stroke.owner_id = Some(owner.clone());
            }
        }

        append_chunk_data(
            result.chunk_id,
            vertices.to_vec(),
            result.indices,
            colors.to_vec(),
            result.strokes,
        );
    }

    for id in found_user_ids.iter() {
        info!("Found user: {}", id);
    }

    info!("Done!");

    return Ok(());
}

fn read_chunk(
    reader: &mut ByteReader,
    found_user_ids: &mut Vec<String>,
) -> Result<ChunkLoadResult, std::io::Error> {
    let chunk_id = read_string(reader)?;
    info!("Reading chunk: {}", chunk_id);

    let num_keys = reader.read_u32()?;
    info!("Has {} keys", num_keys);

    let mut combined_indices = Vec::<u32>::new();
    let mut combined_colors = Vec::<[f32; 4]>::new();
    let mut combined_vertices = Vec::<[f32; 3]>::new();
    let mut strokes = Vec::new();

    for _ in 0..num_keys {
        let is_remote = reader.read_bool()?;
        let mut owner_id: Option<String> = None;
        if is_remote {
            let id = read_string(reader)?;
            owner_id = Some(id.clone());

            if !found_user_ids.contains(&id) {
                found_user_ids.push(id);
            }
        } else {
            if !found_user_ids.contains(&"local".to_string()) {
                found_user_ids.push("local".to_string());
            }
        }

        let num_strokes = reader.read_u32()?;

        info!("Reading {} strokes for {:?}", num_strokes, owner_id);

        for _ in 0..num_strokes {
            let stroke_data = read_stroke(reader, &owner_id, StrokeSource::User)?;

            if stroke_data.metadata.timestamp < 1749429383.719 {
                continue;
            }

            let start_index = u32::try_from(combined_vertices.len()).unwrap();

            let (mut verts, mut colors, indices) = stroke_to_mesh(&stroke_data);

            for i in indices {
                combined_indices.push(i + start_index);
            }

            let mut js_data = JsStrokeData::from_stroke(&stroke_data);

            js_data.vertex_offset = Some(start_index);
            js_data.num_verts = Some(u32::try_from(verts.len()).unwrap());

            combined_vertices.append(&mut verts);
            combined_colors.append(&mut colors);

            strokes.push(js_data);
        }
    }

    Ok(ChunkLoadResult {
        chunk_id: chunk_id,
        indices: combined_indices,
        colors: combined_colors,
        vertices: combined_vertices,
        strokes,
    })
}

pub fn read_stroke(
    reader: &mut ByteReader,
    owner_id: &Option<String>,
    source: StrokeSource,
) -> Result<Stroke, std::io::Error> {
    let id_random = reader.read_u32()?;
    let timestamp = reader.read_f64()?;
    let origin_x = reader.read_f32()?;
    let origin_y = reader.read_f32()?;

    let data_len = reader.read_u32()?;

    let mut slice: Box<[u8]> = vec![0; data_len.try_into().unwrap()].into_boxed_slice();
    reader.read(&mut slice).unwrap();

    let data = StrokeData::parse(slice.to_vec());

    let stroke_data = Stroke {
        data: data.clone(),
        metadata: StrokeMetadata {
            timestamp: timestamp,
            id_random: id_random,
            owner: owner_id.clone(),
            source: source,
            origin: Vec2 {
                x: origin_x,
                y: origin_y,
            },
        },
        mesh: None,
    };

    Ok(stroke_data)
}

fn read_string(reader: &mut ByteReader) -> Result<String, std::io::Error> {
    let len = reader.read_u32()?;
    let mut slice: Box<[u8]> = vec![0; len.try_into().unwrap()].into_boxed_slice();

    reader.read(&mut slice).unwrap();
    let str = str::from_utf8(&slice).unwrap();

    return Ok(str.to_string());
}
