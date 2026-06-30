use std::sync::{LazyLock, Mutex};

use bevy::{
    asset::{Assets, RenderAssetUsages},
    ecs::{
        event::EventReader,
        system::{Commands, Query, ResMut},
    },
    log::info,
    render::{
        mesh::{Indices, Mesh, Mesh2d},
        view::{NoFrustumCulling, RenderLayers},
    },
    sprite::MeshMaterial2d,
};

use crate::{
    CustomMaterial, RENDER_LAYER_ACTIVE_STROKES,
    active_strokes::active_stroke::{ActiveStroke, ActiveStrokeEvent},
};

pub static CURRENT_FRAME_SPAWNED_STROKES: LazyLock<Mutex<Vec<ActiveStroke>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

pub fn spawn_strokes_system(
    mut events: EventReader<ActiveStrokeEvent>,
    mut commands: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    current_strokes: Query<&ActiveStroke>,
) {
    for event in events.read() {
        match event {
            ActiveStrokeEvent::NewPoint(new_point_data) => {
                let mut exists = false;

                for stroke in current_strokes.iter() {
                    if stroke.timestamp == new_point_data.timestamp
                        && stroke.id_random == new_point_data.id_random
                    {
                        exists = true;
                        break;
                    }
                }

                {
                    let mut vec = CURRENT_FRAME_SPAWNED_STROKES.lock().unwrap();
                    for stroke in vec.iter() {
                        if stroke.timestamp == new_point_data.timestamp
                            && stroke.id_random == new_point_data.id_random
                        {
                            exists = true;
                            break;
                        }
                    }
                }

                if exists {
                    continue;
                }

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

                let stroke = ActiveStroke {
                    timestamp: new_point_data.timestamp,
                    id_random: new_point_data.id_random,
                    stroke_origin: new_point_data.stroke_origin,
                    color: new_point_data.color,
                    width: new_point_data.width,
                    points: Vec::new(),
                    is_submitted_to_database: false,
                    pressures: Vec::new(),
                };

                let handle = meshes.add(mesh);

                info!("Spawning new entity for active stroke!");

                {
                    let mut vec = CURRENT_FRAME_SPAWNED_STROKES.lock().unwrap();
                    vec.push(stroke.clone());
                }

                commands.spawn((
                    stroke,
                    NoFrustumCulling {},
                    RenderLayers::from_layers(&[RENDER_LAYER_ACTIVE_STROKES]),
                    Mesh2d(handle),
                    MeshMaterial2d(materials.add(CustomMaterial {})),
                ));
            }
            _ => (),
        }
    }
}

pub fn clear_spawned_strokes() {
    let mut vec = CURRENT_FRAME_SPAWNED_STROKES.lock().unwrap();
    if vec.is_empty() == false {
        info!("Cleared spawned strokes list");
        vec.clear();
    }
}
