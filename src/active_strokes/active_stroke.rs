use bevy::{
    color::Color,
    ecs::{component::Component, event::Event},
    math::Vec2,
};

use crate::stroke::{StrokeSource, StrokeType};

#[derive(Debug, Clone, Component)]
pub struct ActiveStroke {
    pub timestamp: f64,
    pub id_random: u32,
    pub stroke_origin: Vec2,
    pub stroke_type: StrokeType,
    pub width: f32,
    pub points: Vec<Vec2>,
    pub pressures: Vec<f32>,
    pub is_submitted_to_database: bool,
}

#[derive(Debug, Clone)]
pub struct NewPointData {
    pub timestamp: f64,
    pub id_random: u32,
    pub stroke_origin: Vec2,
    pub point: Vec2,
    pub width: f32,
    pub pressure: f32,
    pub stroke_type: StrokeType,
    pub owner: Option<String>,
    pub source: StrokeSource,
}

#[derive(Debug, Clone)]
pub struct StrokeFinishedData {
    pub timestamp: f64,
    pub id_random: u32,
    pub stroke_origin: Vec2,
    pub owner: Option<String>,
    pub source: StrokeSource,
}

#[derive(Debug, Event)]
pub enum ActiveStrokeEvent {
    NewPoint(NewPointData),
    StrokeFinished(StrokeFinishedData),
    DeleteActiveStroke(StrokeFinishedData),
}

#[derive(Debug, Event, Clone)]
pub struct RemoveStrokeEvent {
    pub timestamp: f64,
    pub id_random: u32,
    pub owner: Option<String>,
}
