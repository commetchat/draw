use bevy::{
    app::{App, Plugin},
    ecs::event::Event,
    math::Vec2,
};

pub struct StylusInput;

pub struct PointerData {
    pub pressure: f32,
    pub position: Vec2,
}

#[derive(Event)]
pub enum StylusEvent {
    PointerMove(PointerData),
}

impl Plugin for StylusInput {
    fn build(&self, app: &mut App) {
        app.add_event::<StylusEvent>();
    }
}
