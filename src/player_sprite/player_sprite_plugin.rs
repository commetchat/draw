use bevy::{
    app::{Plugin, PostUpdate, Startup, Update},
    asset::{AssetServer, Assets},
    color::Color,
    ecs::{
        schedule::IntoScheduleConfigs,
        system::{Commands, Query, Res, ResMut},
    },
    math::{Vec2, Vec3, primitives::Ellipse},
    render::{
        mesh::{Mesh, Mesh2d},
        view::RenderLayers,
    },
    sprite::{ColorMaterial, MeshMaterial2d, Sprite},
    transform::components::Transform,
    utils::default,
};

use crate::{
    RENDER_LAYER_SPRITES,
    lerp_transform::TargetTransform,
    player_sprite::{
        get_player_state::{PlayerStateData, get_player_state_system},
        player_sprite::{PlayerSprite, spawn_player_sprite_system},
        zoom_cancel::{ZoomCancel, zoom_cancel_system},
    },
};

pub struct PlayerSpritePlugin;

impl Plugin for PlayerSpritePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<PlayerStateData>();
        app.add_systems(Update, get_player_state_system);
        app.add_systems(PostUpdate, zoom_cancel_system);
        app.add_systems(Update, spawn_player_sprite_system);
    }
}
