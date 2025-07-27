use bevy::{
    app::{App, Plugin},
    color::{Color, ColorToPacked, LinearRgba, Srgba},
    ecs::event::Event,
    math::Vec2,
};
use binary_util::{ByteReader, ByteWriter};
use safe_transmute::{SingleManyGuard, base::transmute_many, transmute_to_bytes};

#[derive(Clone)]
pub enum StrokeType {
    Paint(Color),
    Eraser,
}

#[derive(Clone)]
pub struct StrokeData {
    pub stroke_type: StrokeType,
    pub width: f32,
    pub points: Vec<Vec2>,
    pub pressures: Option<Vec<f32>>,
}

impl StrokeData {
    pub fn new(
        points: Vec<Vec2>,
        pressures: Option<Vec<f32>>,
        width: f32,
        stroke_type: StrokeType,
    ) -> StrokeData {
        StrokeData {
            points: points,
            width: width,
            pressures: pressures,
            stroke_type: stroke_type,
        }
    }

    pub fn parse(data: Vec<u8>) -> StrokeData {
        let mut reader = ByteReader::from(data.as_slice());

        let type_byte = reader.read_u8().unwrap();

        let stroke_type = match type_byte {
            1 => {
                let r = reader.read_u8().unwrap();
                let g = reader.read_u8().unwrap();
                let b = reader.read_u8().unwrap();

                let col = Color::Srgba(Srgba::from_u8_array_no_alpha([r, g, b]));
                StrokeType::Paint(col)
            }
            2 => StrokeType::Eraser,
            _ => {
                panic!();
            }
        };

        let width = reader.read_f32().unwrap();

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

        StrokeData {
            stroke_type: stroke_type,
            width: width,
            points: points,
            pressures: Some(pressures),
        }
    }

    pub fn write_data(&self) -> Vec<u8> {
        let mut writer = ByteWriter::new();

        match self.stroke_type {
            StrokeType::Paint(color) => {
                writer.write_u8(1);
                let color = color.to_srgba().to_u8_array_no_alpha();
                writer.write(&color);
            }
            StrokeType::Eraser => {
                writer.write_u8(2);
            }
        }

        writer.write_f32(self.width);

        writer.write_u32(u32::try_from(self.points.len()).unwrap());

        for point in self.points.iter() {
            writer.write_f32(point.x);
            writer.write_f32(point.y);
        }

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

#[derive(Clone)]
pub struct StrokeMetadata {
    pub timestamp: f64,
    pub id_random: u32,
    pub owner: Option<String>,
    pub origin: Vec2,
}

impl StrokeMetadata {
    pub fn get_id(&self) -> String {
        format!("{}_{}", self.timestamp, self.id_random)
    }
}

#[derive(Clone)]
pub struct StrokeMesh {
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub colors: Vec<[f32; 4]>,
}

impl StrokeMesh {
    pub fn new(vertices: Vec<[f32; 3]>, indices: Vec<u32>, colors: Vec<[f32; 4]>) -> StrokeMesh {
        StrokeMesh {
            vertices: (vertices),
            indices: (indices),
            colors: (colors),
        }
    }

    pub fn from_bytes(
        vertex_data: Vec<u8>,
        index_data: Vec<u32>,
        color_data: Vec<u8>,
    ) -> StrokeMesh {
        let vertices = unsafe {
            match transmute_many::<[f32; 3], SingleManyGuard>(&vertex_data) {
                Ok(data) => data,
                Err(_) => &[],
            }
        }
        .to_vec();

        let colors = unsafe {
            match transmute_many::<[f32; 4], SingleManyGuard>(&color_data) {
                Ok(data) => data,
                Err(_) => &[],
            }
        }
        .to_vec();

        StrokeMesh {
            vertices: vertices,
            colors: colors,
            indices: index_data,
        }
    }

    pub fn write_vertex_data(&self) -> Vec<u8> {
        let bytes = transmute_to_bytes(&self.vertices);
        bytes.to_vec()
    }

    pub fn write_color_data(&self) -> Vec<u8> {
        let bytes = transmute_to_bytes(&self.colors);
        bytes.to_vec()
    }
}

#[derive(Clone)]
pub struct Stroke {
    pub metadata: StrokeMetadata,
    pub data: StrokeData,
    pub mesh: Option<StrokeMesh>,
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
