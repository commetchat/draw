use bevy::{
    asset::{Assets, RenderAssetUsages},
    color::ColorToComponents,
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
    line_builder::LineBuilder,
    mesh_conversion::timestamp_to_z_offset,
};

pub fn update_strokes_system(
    mut events: EventReader<ActiveStrokeEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut current_strokes: Query<(&mut ActiveStroke, &Mesh2d)>,
) {
    if events.is_empty() {
        return;
    }

    info!("Handling new points!");
    for event in events.read() {
        match event {
            ActiveStrokeEvent::NewPoint(new_point_data) => {
                for mut stroke in current_strokes.iter_mut() {
                    if stroke.0.timestamp == new_point_data.timestamp
                        && stroke.0.id_random == new_point_data.id_random
                    {
                        stroke.0.points.push(new_point_data.point);
                        stroke.0.pressures.push(new_point_data.pressure);

                        if stroke.0.points.len() < 2 {
                            continue;
                        }

                        let mesh = meshes.get_mut(stroke.1.id());

                        match mesh {
                            Some(mesh) => {
                                info!("Points: {:?}", stroke.0.points);
                                let mut builder = LineBuilder::new_with(
                                    stroke.0.points.clone(),
                                    stroke.0.pressures.clone(),
                                );

                                builder.width = stroke.0.width;
                                builder.default_color = stroke.0.color;

                                builder.build();
                                let z_offset = timestamp_to_z_offset(stroke.0.timestamp);

                                let mut colors = Vec::<[f32; 4]>::new();
                                let mut vertices = Vec::<[f32; 3]>::new();

                                for p in &builder.vertices {
                                    vertices.push([p.x, p.y, 0.0]);
                                    let mut color =
                                        builder.default_color.to_linear().to_f32_array();

                                    color[3] = z_offset;
                                    colors.push(color);
                                }

                                info!("Generated new mesh with {} verts", vertices.len());

                                if vertices.len() < 3 {
                                    continue;
                                }

                                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
                                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
                                mesh.insert_indices(Indices::U32(builder.indices));
                            }
                            None => (),
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
