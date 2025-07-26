use bevy::{
    color::Color,
    ecs::{component::Component, event::Event},
    math::Vec2,
};

#[derive(Debug, Component)]
pub struct ActiveStroke {
    pub timestamp: f64,
    pub id_random: u32,
    pub stroke_origin: Vec2,
    pub color: Color,
    pub width: f32,
    pub points: Vec<Vec2>,
    pub pressures: Vec<f32>,
}

#[derive(Debug, Clone)]
pub struct NewPointData {
    pub timestamp: f64,
    pub id_random: u32,
    pub stroke_origin: Vec2,
    pub color: Color,
    pub point: Vec2,
    pub width: f32,
    pub pressure: f32,
}

#[derive(Debug)]
pub struct StrokeFinishedData {
    pub timestamp: f64,
    pub id_random: u32,
    pub stroke_origin: Vec2,
}

#[derive(Debug, Event)]
pub enum ActiveStrokeEvent {
    NewPoint(NewPointData),
    StrokeFinished(StrokeFinishedData),
}
