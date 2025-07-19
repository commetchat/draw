use bevy::{
    app::{App, Plugin},
    ecs::event::Event,
    log::info,
    math::Vec2,
};
use binary_util::{ByteReader, ByteWriter};
use safe_transmute::{SingleManyGuard, base::transmute_many, transmute_to_bytes};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum StrokeType {
    paint,
    eraser,
}

pub struct StrokeData {
    binary_data: Option<Vec<u8>>,

    pub points: Option<Vec<Vec2>>,
    pub pressures: Option<Vec<f32>>,
}

struct StrokeHeader {
    num_points: u32,
    num_pressures: u32,
}

impl StrokeData {
    pub fn new(points: Vec<Vec2>, pressures: Option<Vec<f32>>) -> StrokeData {
        StrokeData {
            points: Some(points),
            pressures: pressures,
            binary_data: None,
        }
    }

    pub fn from_bytes(data: Vec<u8>) -> StrokeData {
        StrokeData {
            binary_data: Some(data),
            points: None,
            pressures: None,
        }
    }

    pub fn parse(&mut self) {
        let data = self.binary_data.as_ref().unwrap();
        let mut reader = ByteReader::from(data.as_slice());

        let mut points = Vec::new();

        for _ in 0..reader.read_u32().unwrap() {
            let x = reader.read_f32().unwrap();
            let y = reader.read_f32().unwrap();
            points.push(Vec2 { x: x, y: y });
        }

        let mut pressures = Vec::new();

        for _ in 0..reader.read_u32().unwrap() {
            let p = reader.read_f32().unwrap();
            pressures.push(p);
        }

        self.points = Some(points);
        self.pressures = Some(pressures);
    }

    pub fn write_data(&self) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        match &self.points {
            Some(points) => {
                writer.write_u32(u32::try_from(points.len()).unwrap());

                for point in points.iter() {
                    writer.write_f32(point.x);
                    writer.write_f32(point.y);
                }
            }
            None => {
                writer.write_u32(0);
            }
        };

        match &self.pressures {
            Some(pressures) => {
                writer.write_u32(u32::try_from(pressures.len()).unwrap());

                for pressure in pressures.iter() {
                    writer.write_f32(*pressure);
                }
            }
            None => {
                writer.write_u32(0);
            }
        };

        writer.as_slice().to_vec()
    }
}

pub struct StrokeMetadata {
    pub timestamp: f64,
    pub id_random: u32,
    pub owner: Option<String>,
    pub origin: Vec2,
}

impl StrokeMetadata {
    pub fn get_id(&self) -> String {
        match &self.owner {
            Some(owner) => format!("{}_{}_{owner}", self.timestamp, self.id_random),
            None => format!("{}_{}", self.timestamp, self.id_random),
        }
    }
}

pub struct StrokeMesh {
    binary_data: Option<Vec<u8>>,
    pub vertices: Option<Vec<[f32; 3]>>,
    pub indices: Option<Vec<u32>>,
    pub colors: Option<Vec<[f32; 4]>>,
}

impl StrokeMesh {
    pub fn new(vertices: Vec<[f32; 3]>, indices: Vec<u32>, colors: Vec<[f32; 4]>) -> StrokeMesh {
        StrokeMesh {
            vertices: Some(vertices),
            indices: Some(indices),
            colors: Some(colors),
            binary_data: None,
        }
    }

    pub fn from_bytes(data: Vec<u8>) -> StrokeMesh {
        StrokeMesh {
            binary_data: Some(data),
            vertices: None,
            colors: None,
            indices: None,
        }
    }

    pub fn load(&mut self) {
        if !(self.vertices.is_none() && self.binary_data.is_some()) {
            return;
        }

        let data = self.binary_data.as_ref().unwrap();
        let mut reader = ByteReader::from(data.as_slice());

        let num_bytes = reader.read_u32().unwrap();
        let mut buf = vec![0u8; num_bytes.try_into().unwrap()];
        reader.read(&mut buf);

        let vertices = if (buf.len() != 0) {
            unsafe { transmute_many::<[f32; 3], SingleManyGuard>(&buf).unwrap() }.to_vec()
        } else {
            Vec::new()
        };

        let num_bytes = reader.read_u32().unwrap();
        let mut buf = vec![0u8; num_bytes.try_into().unwrap()];
        reader.read(&mut buf);

        let indices = if (buf.len() != 0) {
            unsafe { transmute_many::<u32, SingleManyGuard>(&buf).unwrap() }.to_vec()
        } else {
            Vec::new()
        };

        let num_bytes = reader.read_u32().unwrap();
        let mut buf = vec![0u8; num_bytes.try_into().unwrap()];
        reader.read(&mut buf);

        let colors = if (buf.len() != 0) {
            unsafe { transmute_many::<[f32; 4], SingleManyGuard>(&buf).unwrap() }.to_vec()
        } else {
            Vec::new()
        };

        self.vertices = Some(vertices);
        self.indices = Some(indices);
        self.colors = Some(colors);
    }

    pub fn write_data(&self) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        let num_verts = self.vertices.as_ref().unwrap().len();

        let data = match &self.vertices {
            Some(bytes) => bytes,
            None => todo!(),
        };

        let bytes = transmute_to_bytes(data);
        writer.write_u32(u32::try_from(bytes.len()).unwrap());
        writer.write(bytes);

        let data = match &self.indices {
            Some(bytes) => bytes,
            None => todo!(),
        };
        let bytes = transmute_to_bytes(data);
        writer.write_u32(u32::try_from(bytes.len()).unwrap());
        writer.write(bytes);

        let data = match &self.colors {
            Some(bytes) => bytes,
            None => todo!(),
        };
        let bytes = transmute_to_bytes(data);
        writer.write_u32(u32::try_from(bytes.len()).unwrap());
        writer.write(bytes);

        writer.as_slice().to_vec()
    }
}

pub struct Stroke {
    pub metadata: StrokeMetadata,
    pub data: StrokeData,
    pub mesh: StrokeMesh,
}

pub struct Strokes;

#[derive(Event)]
pub enum StrokeEvent {
    StrokeFinished(Stroke),
}

impl Plugin for Strokes {
    fn build(&self, app: &mut App) {
        app.add_event::<StrokeEvent>();
    }
}
