use std::collections::VecDeque;

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        mesh::{self, Indices, VertexAttributeValues},
        view::NoFrustumCulling,
    },
};

use crate::{
    CustomMaterial,
    chunks::ChunkEvent,
    database::{LOAD_STROKE_QUEUE, web_database::load_strokes_for_chunk},
    stroke::{self, Stroke},
    utils::now,
};

#[derive(Component, Default)]
pub struct Chunk {
    has_requested_db_chunks: bool,
    finished_loading: bool,
    chunk_id: String,
}

pub fn chunk_spawn_system(
    chunks: Query<(Entity, &Chunk, &Mesh2d)>,
    mut events: EventReader<ChunkEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    mut commands: Commands,
) {
    let _ = events;
    for event in events.read() {
        match event {
            ChunkEvent::Visible(id) => {
                let mut mesh = Mesh::new(
                    bevy::render::mesh::PrimitiveTopology::TriangleList,
                    RenderAssetUsages::all(),
                );
                let verts: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0], [0.0, 5.0, 0.0], [0.0, 5.0, 5.0]];
                let colors: Vec<[f32; 4]> = vec![
                    [0.0, 0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                    [0.0, 0.0, 0.0, 0.0],
                ];

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
                mesh.insert_indices(Indices::U32(vec![0, 1, 2]));

                let handle = meshes.add(mesh);
                info!("Spawning chunk: {}  {}", id, handle.id());

                commands.spawn((
                    Chunk {
                        has_requested_db_chunks: false,
                        chunk_id: id.clone(),
                        finished_loading: false,
                    },
                    NoFrustumCulling {},
                    Mesh2d(handle),
                    MeshMaterial2d(materials.add(CustomMaterial {})),
                ));
            }
            ChunkEvent::NotVisible(id) => {
                for chunk in chunks.iter() {
                    if &chunk.1.chunk_id == id {
                        commands.entity(chunk.0).despawn();
                    }

                    let mesh = chunk.2;
                    meshes.remove(mesh.id());
                }
            }
        }
    }
}

pub fn update_chunk_system(
    mut chunks: Query<(&mut Chunk, &mut Mesh2d)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let a = now();
    for mut chunk in chunks.iter_mut() {
        if chunk.0.has_requested_db_chunks == false {
            chunk.0.has_requested_db_chunks = true;
            load_strokes_for_chunk(chunk.0.chunk_id.clone());
        }
    }

    let mut map = match LOAD_STROKE_QUEUE.lock() {
        Ok(map) => map,
        Err(_) => {
            info!("Failed to acquire lock!");
            return;
        }
    };

    for mut chunk in chunks.iter_mut() {
        if (chunk.0.finished_loading) {
            continue;
        }

        let queue = map.get_mut(&chunk.0.chunk_id);

        let queue = match queue {
            Some(queue) => queue,
            None => continue,
        };

        if (queue.is_empty()) {
            continue;
        }

        let handle = chunk.1;
        let handle = &handle.0;
        let mesh = match meshes.get_mut(handle.id()) {
            Some(mesh) => mesh,
            None => {
                info!("Failed to get mesh from handle! {}", handle.id());
                continue;
            }
        };

        handle_queue(mesh, queue);
        chunk.0.finished_loading = true;
    }

    let b = now();
}

fn handle_queue(mesh: &mut Mesh, queue: &mut VecDeque<Stroke>) {
    let mut verts = {
        let verts = mesh.attribute(Mesh::ATTRIBUTE_POSITION);

        match verts {
            Some(verts) => match verts {
                VertexAttributeValues::Float32x3(items) => items.clone(),
                _ => Vec::new(),
            },
            None => Vec::new(),
        }
    };

    let mut colors = {
        let colors = mesh.attribute_mut(Mesh::ATTRIBUTE_COLOR);

        match colors {
            Some(colors) => match colors {
                VertexAttributeValues::Float32x4(items) => items.clone(),
                _ => Vec::new(),
            },
            None => Vec::new(),
        }
    };

    let mut indices = match mesh.indices_mut() {
        Some(indices) => {
            if let Indices::U32(indices) = indices {
                indices.clone()
            } else {
                Vec::new()
            }
        }
        None => Vec::new(),
    };

    loop {
        let mut stroke = match queue.pop_front() {
            Some(stroke) => stroke,
            None => break,
        };

        stroke.mesh.load();

        let num_verts = u32::try_from(verts.len()).unwrap();

        verts.append(&mut stroke.mesh.vertices.unwrap());
        colors.append(&mut stroke.mesh.colors.unwrap());
        for i in stroke.mesh.indices.unwrap().iter() {
            indices.push(i + num_verts)
        }
    }

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));
}
