use bevy::{
    asset::AssetServer,
    ecs::{
        component::Component,
        event::EventReader,
        system::{Commands, Res, Single},
    },
    math::Vec2,
    render::view::{NoFrustumCulling, RenderLayers},
    sprite::Sprite,
    transform::components::Transform,
    utils::default,
    window::Window,
};

use crate::{
    RENDER_LAYER_SPRITES,
    lerp_transform::TargetTransform,
    networking::{connected_peers::PeerConnectedEvent, network_owned::NetworkOwned},
    player_sprite::zoom_cancel::ZoomCancel,
    utils::get_random_uint32,
};

#[derive(Component, Default)]
pub struct PlayerSprite {
    position: Vec2,
    rotation: f32,
    zoom: f32,
}

pub fn spawn_player_sprite_system(
    mut events: EventReader<PeerConnectedEvent>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window: Single<&mut Window>,
) {
    for event in events.read() {
        let images = [
            "embedded://characters/character_green_jump.png",
            "embedded://characters/character_pink_duck.png",
            "embedded://characters/character_purple_walk_a.png",
            "embedded://characters/character_yellow_walk_b.png",
        ];

        let i = get_random_uint32();
        let index = i % u32::try_from(images.len()).unwrap();
        let img = images[usize::try_from(index).unwrap()];

        let img = asset_server.load(img);

        let window_scale = window.resolution.base_scale_factor();

        let size = window_scale * 64.0;

        commands.spawn((
            Sprite {
                image: img,
                custom_size: Some(Vec2 { x: size, y: size }),
                ..default()
            },
            NetworkOwned {
                owner: event.id.clone(),
            },
            RenderLayers::from_layers(&[RENDER_LAYER_SPRITES]),
            TargetTransform {
                transform: Transform::default(),
                speed: 10.0,
                do_scale: false,
            },
            NoFrustumCulling,
            ZoomCancel,
            PlayerSprite::default(),
        ));
    }
}
