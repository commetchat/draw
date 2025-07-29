use bevy::{
    color::palettes::css::{BLUE, GREEN, PURPLE, RED},
    prelude::*,
    window::PrimaryWindow,
};

use crate::{
    chunks::chunk::{
        append_stroke_system, chunk_draw_system, chunk_spawn_system, remove_chunk_verts_system,
        update_chunk_system,
    },
    utils::DEBUG_DRAW,
};

mod chunk;

pub struct ChunksPlugin;

#[derive(Component, Default)]
pub struct ChunkController {
    loaded_chunks: Vec<String>,
}

#[derive(Event)]
pub enum ChunkEvent {
    Visible(String, Vec2),
    NotVisible(String),
}

const CHUNK_SIZE: f32 = 800.0;

impl Plugin for ChunksPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChunkEvent>();
        app.add_systems(Update, chunk_spawn_system);
        app.add_systems(Update, update_chunk_system);
        app.add_systems(Update, chunk_draw_system);
        app.add_systems(Update, append_stroke_system);
        app.add_systems(Update, remove_chunk_verts_system);
        app.add_systems(
            PostUpdate,
            show_chunks.after(TransformSystem::TransformPropagate),
        );
    }
}

pub fn position_to_chunk_position(position: Vec2) -> Vec2 {
    return Vec2 {
        x: (position.x / CHUNK_SIZE).floor() * CHUNK_SIZE,
        y: (position.y / CHUNK_SIZE).floor() * CHUNK_SIZE,
    };
}

pub fn position_to_chunk_id(position: Vec2) -> String {
    let chunk_pos = Vec2 {
        x: (position.x / CHUNK_SIZE).floor(),
        y: (position.y / CHUNK_SIZE).floor(),
    };

    return format!("{}_{}", chunk_pos.x, chunk_pos.y);
}

fn show_chunks(
    mut camera_query: Single<(&Camera, &GlobalTransform, &mut ChunkController)>,
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut events: EventWriter<ChunkEvent>,
    mut gizmos: Gizmos,
) {
    let camera = camera_query.0;
    let camera_transform = camera_query.1;

    let size = window.physical_size();

    let mut padding = -Vec2::new(300.0, 300.0);

    if DEBUG_DRAW {
        padding *= -1.0;
    }

    let corner = Vec2 {
        x: size.x as f32,
        y: size.y as f32,
    } - padding;

    let mut position = padding;

    let scale = 1.0 / camera_transform.scale().x;

    let mut visible_chunk_ids = vec![];

    let camera_position = camera_transform.translation();

    // always get a 3x3 grid around the camera
    for x in -1..=1 {
        for y in -1..=1 {
            let mut pos = Vec2 {
                x: camera_position.x,
                y: camera_position.y,
            };
            pos.x += (CHUNK_SIZE * 0.9) * (x as f32);
            pos.y += (CHUNK_SIZE * 0.9) * (y as f32);

            let id = position_to_chunk_id(pos);
            let chunk_pos = position_to_chunk_position(pos);

            if visible_chunk_ids.iter().any(|f: &(String, Vec2)| f.0 == id) == false {
                visible_chunk_ids.push((id, chunk_pos));
            }

            if DEBUG_DRAW {
                gizmos.circle_2d(pos, 10.0, PURPLE);
            }
        }
    }

    while position.x < corner.x {
        while position.y < corner.y {
            if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, position) {
                let id = position_to_chunk_id(world_pos);
                let pos = position_to_chunk_position(world_pos);

                if visible_chunk_ids.iter().any(|f: &(String, Vec2)| f.0 == id) == false {
                    visible_chunk_ids.push((id, pos));
                }

                if DEBUG_DRAW {
                    gizmos.circle_2d(world_pos, 10.0, BLUE);
                }
            }

            position.y += 0.5 * CHUNK_SIZE * scale;
        }

        position.y = padding.y;
        position.x += 0.5 * CHUNK_SIZE * scale;
    }

    let mut position = Vec2 { x: 0.0, y: 0.0 };

    for id in camera_query.2.loaded_chunks.iter() {
        if visible_chunk_ids
            .iter()
            .any(|f: &(String, Vec2)| f.0 == *id)
            == false
        {
            events.write(ChunkEvent::NotVisible(id.clone()));
        }
    }

    for chunk in visible_chunk_ids.iter() {
        if camera_query.2.loaded_chunks.contains(&chunk.0) == false {
            events.write(ChunkEvent::Visible(chunk.0.clone(), chunk.1));
        }
    }

    camera_query.2.loaded_chunks = visible_chunk_ids.iter().map(|f| f.0.clone()).collect();
}
