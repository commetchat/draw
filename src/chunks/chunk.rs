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
    database::{LOAD_MESH_QUEUE, web_database::load_mesh_for_chunk, web_stroke_data::JsMeshData},
    stroke::{self, Stroke, StrokeMesh},
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
            load_mesh_for_chunk(chunk.0.chunk_id.clone());
        }
    }

    let mut map = match LOAD_MESH_QUEUE.lock() {
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

fn handle_queue(mesh: &mut Mesh, queue: &mut VecDeque<JsMeshData>) {
    let mut stroke = match queue.pop_front() {
        Some(stroke) => stroke,
        None => return,
    };

    let stroke_mesh =
        StrokeMesh::from_bytes(stroke.vertex_data, stroke.index_data, stroke.color_data);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, stroke_mesh.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, stroke_mesh.colors);
    mesh.insert_indices(Indices::U32(stroke_mesh.indices));
}
