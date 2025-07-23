use std::sync::Mutex;

use bevy::{
    app::Plugin,
    color::ColorToComponents,
    log::{info, tracing_subscriber::fmt::time},
    math::Vec2,
};
use binary_util::ByteReader;
use safe_transmute::{SingleManyGuard, transmute_many, transmute_to_bytes};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    BACKGROUND,
    database::{web_database::set_initial_chunk_state, web_stroke_data::JsStrokeData},
    line_builder::{LineBuilder, LineCapMode, LineJointMode},
    stroke::{Stroke, StrokeData, StrokeMetadata},
};

pub struct LoadFile;

#[wasm_bindgen]
pub fn load_file(bytes: Vec<u8>) {
    match load(bytes) {
        Ok(_) => (),
        Err(_) => {
            info!("Failed to load from file!");
        }
    }
}

fn load(bytes: Vec<u8>) -> Result<(), std::io::Error> {
    let mut reader = ByteReader::from(bytes);
    info!("Reading save file");

    let magic = reader.read_u32()?;
    let version = reader.read_u32()?;
    info!("Magic: {}", magic);
    info!("File version: {}", version);
    while reader.peek_ahead(1).is_ok() {
        let chunk_id = read_string(&mut reader)?;

        info!("Reading chunk: {}", chunk_id);

        let num_keys = reader.read_u32()?;
        info!("Has {} keys", num_keys);

        let mut indices = Vec::<u32>::new();
        let mut colors = Vec::<[f32; 4]>::new();
        let mut vertices = Vec::<[f32; 3]>::new();
        let mut strokes = Vec::new();

        for _ in 0..num_keys {
            let is_remote = reader.read_bool()?;
            let mut owner_id: Option<String> = None;
            if is_remote {
                owner_id = Some(read_string(&mut reader)?);
            }

            let num_strokes = reader.read_u32()?;

            info!("Reading {} strokes for {:?}", num_strokes, owner_id);

            for _ in 0..num_strokes {
                let id_random = reader.read_u32()?;
                let timestamp = reader.read_f64()?;
                let origin_x = reader.read_f32()?;
                let origin_y = reader.read_f32()?;

                let data_len = reader.read_u32()?;

                let mut slice: Box<[u8]> = vec![0; data_len.try_into().unwrap()].into_boxed_slice();
                reader.read(&mut slice).unwrap();

                let data = StrokeData::parse(slice.to_vec());
                let mut builder = LineBuilder::new();

                let stroke_data = JsStrokeData::from_stroke(&Stroke {
                    data: data.clone(),
                    metadata: StrokeMetadata {
                        timestamp: timestamp,
                        id_random: id_random,
                        owner: owner_id.clone(),
                        origin: Vec2 {
                            x: origin_x,
                            y: origin_y,
                        },
                    },
                    mesh: None,
                });

                let start_index = u32::try_from(vertices.len()).unwrap();

                builder.end_cap_mode = LineCapMode::Round;
                builder.begin_cap_mode = LineCapMode::Round;
                builder.joint_mode = LineJointMode::Round;

                builder.width = 1.0;

                builder.width = data.width;
                builder.default_color = match data.stroke_type {
                    crate::stroke::StrokeType::Paint(color) => color,
                    crate::stroke::StrokeType::Eraser => BACKGROUND,
                };
                builder.points = data.points;
                builder.pressures = match data.pressures {
                    Some(pressure) => pressure,
                    None => Vec::new(),
                };

                let mut z_offset = timestamp;

                z_offset -= 1740000000.0;
                z_offset *= 0.00000001;

                builder.build();
                vertices.reserve(builder.vertices.len());
                colors.reserve(builder.colors.len());

                strokes.push(stroke_data);

                for p in &builder.vertices {
                    vertices.push([p.x + origin_x, p.y + origin_y, 0.0]);
                    let mut color = builder.default_color.to_linear().to_f32_array(); // LinearRgba::from_u8_array_no_alpha().to_f32_array();

                    color[3] = z_offset as f32;
                    colors.push(color);
                }

                for i in builder.indices {
                    indices.push(i + start_index);
                }
            }
        }

        let vertices = transmute_to_bytes(&vertices);
        let colors = transmute_to_bytes(&colors);

        set_initial_chunk_state(
            chunk_id,
            vertices.to_vec(),
            indices,
            colors.to_vec(),
            strokes,
        );
    }

    info!("Done!");

    return Ok(());
}

fn read_string(reader: &mut ByteReader) -> Result<String, std::io::Error> {
    let len = reader.read_u32()?;
    let mut slice: Box<[u8]> = vec![0; len.try_into().unwrap()].into_boxed_slice();

    reader.read(&mut slice).unwrap();
    let str = str::from_utf8(&slice).unwrap();

    return Ok(str.to_string());
}
