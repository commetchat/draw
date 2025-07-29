use std::collections::VecDeque;

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::BLUE,
    prelude::*,
    render::{
        mesh::{self, Indices, VertexAttributeValues},
        view::{NoFrustumCulling, RenderLayers},
    },
};

use crate::{
    CustomMaterial, RENDER_LAYER_BATCH_STROKES,
    active_strokes::active_stroke::{ActiveStrokeEvent, StrokeFinishedData},
    chunks::{CHUNK_SIZE, ChunkEvent},
    database::{
        APPEND_STROKE_DATAS, CHUNKS_NEED_RELOADING, DATABASE_READY, LOAD_MESH_QUEUE,
        REMOVE_CHUNK_VERTS, web_database::load_mesh_for_chunk, web_stroke_data::JsMeshData,
    },
    retained_view::copy_camera::RetainedViewEvent,
    stroke::{self, Stroke, StrokeMesh},
    utils::{DEBUG_DRAW, now},
};

#[derive(Component, Default)]
pub struct Chunk {
    position: Vec2,
    has_requested_db_chunks: bool,
    finished_loading: bool,
    chunk_id: String,
}

pub fn chunk_draw_system(chunks: Query<(Entity, &Chunk, &Mesh2d)>, mut gizmos: Gizmos) {
    for chunk in chunks.iter() {
        let pos = chunk.1.position;
        let padding = Vec2 { x: 20.0, y: 20.0 };

        if DEBUG_DRAW {
            gizmos.line_2d(
                pos + padding,
                pos + Vec2 {
                    x: CHUNK_SIZE - padding.x,
                    y: padding.y,
                },
                Color::LinearRgba(LinearRgba {
                    red: 0.5,
                    green: 0.5,
                    blue: 0.0,
                    alpha: 1.0,
                }),
            );

            gizmos.line_2d(
                pos + padding,
                pos + Vec2 {
                    x: padding.x,
                    y: CHUNK_SIZE - padding.y,
                },
                Color::LinearRgba(LinearRgba {
                    red: 0.5,
                    green: 0.0,
                    blue: 0.5,
                    alpha: 1.0,
                }),
            );
        }
    }
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
            ChunkEvent::Visible(id, position) => {
                let mut mesh = Mesh::new(
                    bevy::render::mesh::PrimitiveTopology::TriangleList,
                    RenderAssetUsages::all(),
                );
                let verts: Vec<[f32; 3]> = vec![[0.0, 0.0, 0.0]];
                let colors: Vec<[f32; 4]> = vec![[0.0, 0.0, 0.0, 0.0]];

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
                mesh.insert_indices(Indices::U32(vec![0, 0, 0]));

                let handle = meshes.add(mesh);
                info!("Spawning chunk: {}  {}", id, handle.id());

                commands.spawn((
                    Chunk {
                        position: *position,
                        has_requested_db_chunks: false,
                        chunk_id: id.clone(),
                        finished_loading: false,
                    },
                    RenderLayers::from_layers(&[RENDER_LAYER_BATCH_STROKES]),
                    NoFrustumCulling {},
                    Mesh2d(handle),
                    MeshMaterial2d(materials.add(CustomMaterial {})),
                ));
            }
            ChunkEvent::NotVisible(id) => {
                for chunk in chunks.iter() {
                    if &chunk.1.chunk_id == id {
                        commands.entity(chunk.0).despawn();

                        let mesh = chunk.2;
                        meshes.remove(mesh.id());
                    }
                }
            }
        }
    }
}

pub fn append_stroke_system(
    mut chunks: Query<(&mut Chunk, &mut Mesh2d)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut stroke_events: EventWriter<ActiveStrokeEvent>,
    mut render_events: EventWriter<RetainedViewEvent>,
) {
    let ready = DATABASE_READY.lock().unwrap();
    if *ready == false {
        return;
    }

    let mut did_change_data = false;

    let mut map = APPEND_STROKE_DATAS.lock().unwrap();

    for mut chunk in chunks.iter_mut() {
        let queue = map.get_mut(&chunk.0.chunk_id);

        let queue = match queue {
            Some(queue) => queue,
            None => {
                continue;
            }
        };

        while let Some(item) = queue.pop_front() {
            let handle = &chunk.1;
            let handle = &handle.0;

            let mesh = match meshes.get_mut(handle.id()) {
                Some(mesh) => mesh,
                None => {
                    info!("Failed to get mesh from handle! {}", handle.id());
                    continue;
                }
            };

            let mut verts = match (mesh.attribute(Mesh::ATTRIBUTE_POSITION)) {
                Some(verts) => match (verts) {
                    VertexAttributeValues::Float32x3(items) => items,
                    _ => {
                        continue;
                    }
                },
                None => continue,
            };

            let mut colors = match (mesh.attribute(Mesh::ATTRIBUTE_COLOR)) {
                Some(verts) => match (verts) {
                    VertexAttributeValues::Float32x4(items) => items,
                    _ => {
                        continue;
                    }
                },
                None => continue,
            };

            let mut indices = match mesh.indices() {
                Some(indices) => match indices {
                    Indices::U32(items) => items,
                    _ => {
                        continue;
                    }
                },
                None => {
                    continue;
                }
            };

            let mut verts = verts.clone();
            let mut indices = indices.clone();
            let mut colors = colors.clone();

            let mut stroke_mesh = StrokeMesh::from_bytes(
                item.vertex_data.unwrap(),
                item.index_data.unwrap(),
                item.color_data.unwrap(),
            );

            if verts.len() == 1 && item.vertex_offset == Some(0) {
                verts = stroke_mesh.vertices;
                indices = stroke_mesh.indices;
                colors = stroke_mesh.colors;
            } else if verts.len() == usize::try_from(item.vertex_offset.unwrap()).unwrap() {
                verts.append(&mut stroke_mesh.vertices);
                colors.append(&mut stroke_mesh.colors);
                indices.append(&mut stroke_mesh.indices);
            } else {
                info!("Unexpected vertex count in mesh! something is not right!");
            }

            if verts.len() == 0 {
                info!("Number of verts is ZERO, somethings gone wrong!");
                return;
            }

            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
            mesh.insert_indices(Indices::U32(indices));

            stroke_events.write(ActiveStrokeEvent::DeleteActiveStroke(StrokeFinishedData {
                timestamp: item.timestamp,
                id_random: item.id_random,
                owner: item.owner_id,
                stroke_origin: Vec2 {
                    x: item.origin_x,
                    y: item.origin_y,
                },
            }));

            did_change_data = true;
        }
    }

    map.clear();

    if did_change_data {
        render_events.write(RetainedViewEvent::UpdateFrame);
    }
}

pub fn update_chunk_system(
    mut chunks: Query<(&mut Chunk, &mut Mesh2d)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_events: EventWriter<RetainedViewEvent>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let ready = DATABASE_READY.lock().unwrap();
    if *ready == false {
        return;
    }

    let mut needs_reloading = CHUNKS_NEED_RELOADING.lock().unwrap();

    let a = now();
    for mut chunk in chunks.iter_mut() {
        let chunk_needs_reloading = needs_reloading.contains(&chunk.0.chunk_id);
        let i = needs_reloading.iter().position(|r| r == &chunk.0.chunk_id);

        if chunk.0.has_requested_db_chunks == false || i.is_some() {
            if (i.is_some()) {
                needs_reloading.remove(i.unwrap());
                info!("Reloading chunk: {}", chunk.0.chunk_id);
            }

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
        render_events.write(RetainedViewEvent::UpdateFrame);
        chunk.0.finished_loading = true;
    }

    let b = now();
}

pub fn remove_chunk_verts_system(
    mut chunks: Query<(&mut Chunk, &mut Mesh2d)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut render_events: EventWriter<RetainedViewEvent>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let ready = DATABASE_READY.lock().unwrap();
    if *ready == false {
        return;
    }

    let mut changed = false;

    let mut queue = REMOVE_CHUNK_VERTS.lock().unwrap();

    while let Some(item) = queue.pop_front() {
        info!("Removing verts from chunk!");
        for chunk in chunks.iter_mut() {
            if chunk.0.chunk_id != item.chunk_key {
                continue;
            }

            if let Some(mesh) = meshes.get_mut(chunk.1.id()) {
                let mut verts = match (mesh.attribute(Mesh::ATTRIBUTE_POSITION)) {
                    Some(verts) => match (verts) {
                        VertexAttributeValues::Float32x3(items) => items.clone(),
                        _ => {
                            continue;
                        }
                    },
                    None => continue,
                };

                let num_verts = verts.len();
                if verts.len() < usize::try_from(item.offset + item.num_verts).unwrap() {
                    info!("Not enough verts to remove!");
                    continue;
                }

                for i in 0..item.num_verts {
                    let index = item.offset + i;
                    verts[index as usize] = [0.0, 0.0, 0.0];
                }

                info!("Removed mesh!");

                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
                changed = true;
            }
        }
    }

    if changed {
        render_events.write(RetainedViewEvent::UpdateFrame);
    }
}

fn handle_queue(mesh: &mut Mesh, queue: &mut VecDeque<JsMeshData>) {
    let mut stroke = match queue.pop_front() {
        Some(stroke) => stroke,
        None => return,
    };

    if stroke.vertex_data.is_empty() || stroke.index_data.is_empty() || stroke.color_data.is_empty()
    {
        return;
    }

    let stroke_mesh =
        StrokeMesh::from_bytes(stroke.vertex_data, stroke.index_data, stroke.color_data);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, stroke_mesh.vertices);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, stroke_mesh.colors);
    mesh.insert_indices(Indices::U32(stroke_mesh.indices));
}
