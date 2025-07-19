use bevy::{
    app::{App, Plugin},
    ecs::event::Event,
    math::Vec2,
};
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Eq, PartialEq, Debug, Clone)]
pub enum StrokeType {
    paint,
    eraser,
}

pub struct StrokeData {
    pub points: Vec<Vec2>,
    pub pressures: Option<Vec<f32>>,
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

pub struct Stroke {
    pub metadata: StrokeMetadata,
    pub data: StrokeData,
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
