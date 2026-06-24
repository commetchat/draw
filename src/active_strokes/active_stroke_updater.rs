use bevy::{
    asset::Assets,
    color::{ColorToComponents, Saturation},
    ecs::{
        entity::Entity,
        event::EventReader,
        system::{Commands, Query, ResMut},
    },
    log::info,
    math::Vec2,
    render::mesh::{Indices, Mesh, Mesh2d},
};

use crate::{
    active_strokes::{
        active_stroke::{ActiveStroke, ActiveStrokeEvent, RemoveStrokeEvent},
        active_stroke_lifetime::ActiveStrokeLifetime,
    },
    database::{
        web_database::{delete_stroke, store_multiple_strokes},
        web_stroke_data::JsStrokeData,
    },
    line_builder::LineBuilder,
    mesh_conversion::timestamp_to_z_offset,
    stroke::{Stroke, StrokeData, StrokeMesh, StrokeMetadata},
    utils::DEBUG_DRAW,
};

pub fn update_strokes_system(
    mut events: EventReader<ActiveStrokeEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut current_strokes: Query<(Entity, &mut ActiveStroke, &Mesh2d)>,
    mut commands: Commands,
) {
    if events.is_empty() {
        return;
    }

    for event in events.read() {
        match event {
            ActiveStrokeEvent::NewPoint(new_point_data) => {
                for mut stroke in current_strokes.iter_mut() {
                    if stroke.1.timestamp == new_point_data.timestamp
                        && stroke.1.id_random == new_point_data.id_random
                    {
                        stroke
                            .1
                            .points
                            .push(new_point_data.point - new_point_data.stroke_origin);

                        stroke.1.pressures.push(new_point_data.pressure);

                        if stroke.1.points.len() < 2 {
                            continue;
                        }

                        let mesh = meshes.get_mut(stroke.2.id());

                        match mesh {
                            Some(mesh) => {
                                let (verts, indices, colors) =
                                    active_stroke_to_mesh(&stroke.1, DEBUG_DRAW);

                                if verts.len() < 3 {
                                    continue;
                                }

                                mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, verts);
                                mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
                                mesh.insert_indices(Indices::U32(indices));
                            }
                            None => (),
                        }
                    }
                }
            }
            ActiveStrokeEvent::StrokeFinished(data) => {
                info!("Got Stroke Finished Event");
                for mut stroke in current_strokes.iter_mut() {
                    if stroke.1.timestamp == data.timestamp && stroke.1.id_random == data.id_random
                    {
                        if stroke.1.points.len() < 2 {
                            continue;
                        }

                        if stroke.1.is_submitted_to_database {
                            continue;
                        }

                        let (verts, indices, colors) = active_stroke_to_mesh(&stroke.1, false);

                        let mut width = stroke.1.width;

                        let mut pressure_changes = false;

                        for i in 1..stroke.1.pressures.len() {
                            let prev = stroke.1.pressures.get(i - 1).unwrap();
                            let curr = stroke.1.pressures.get(i).unwrap();
                            if prev != curr {
                                pressure_changes = true;
                                break;
                            }
                        }

                        let mut pressures = Some(stroke.1.pressures.clone());

                        if pressure_changes == false {
                            width *= stroke.1.pressures.get(0).unwrap();
                            pressures = None;
                        }

                        let stroke_data = Stroke {
                            metadata: StrokeMetadata {
                                timestamp: data.timestamp,
                                id_random: data.id_random,
                                origin: data.stroke_origin,
                                source: data.source.clone(),
                                owner: data.owner.clone(),

                            },
                            data: StrokeData {
                                stroke_type: crate::stroke::StrokeType::Paint(stroke.1.color),
                                width: stroke.1.width,
                                points: stroke.1.points.clone(),
                                pressures: pressures,
                            },
                            mesh: Some(StrokeMesh {
                                vertices: verts,
                                indices: indices,
                                colors: colors,
                            }),
                        };

                        stroke.1.is_submitted_to_database = true;

                        let data = JsStrokeData::from_stroke(&stroke_data);

                        store_multiple_strokes(vec![data]);
                    }
                }
            }
            ActiveStrokeEvent::DeleteActiveStroke(data) => {
                for stroke in current_strokes.iter_mut() {
                    if stroke.1.timestamp == data.timestamp && stroke.1.id_random == data.id_random
                    {
                        commands.entity(stroke.0).insert(
                            ActiveStrokeLifetime {
                                remaining_life: 1.0,
                            },
                        );
                    }
                }
            }
        }
    }
}

pub fn remove_strokes_system(mut events: EventReader<RemoveStrokeEvent>) {
    if events.is_empty() {
        return;
    }

    for event in events.read() {
        let meta = StrokeMetadata {
            timestamp: event.timestamp,
            id_random: event.id_random,
            source: crate::stroke::StrokeSource::User,
            owner: None,
            origin: Vec2::ZERO,
        };

        let id = meta.get_id();

        info!("Removing: {}", id);

        delete_stroke(id);
    }
}

fn active_stroke_to_mesh(
    stroke: &ActiveStroke,
    debug: bool,
) -> (
    std::vec::Vec<[f32; 3]>,
    std::vec::Vec<u32>,
    std::vec::Vec<[f32; 4]>,
) {
    let mut builder = LineBuilder::new_with(stroke.points.clone(), stroke.pressures.clone());

    let z_offset = timestamp_to_z_offset(stroke.timestamp);

    let mut colors = Vec::<[f32; 4]>::new();
    let mut vertices = Vec::<[f32; 3]>::new();

    builder.width = stroke.width;

    builder.default_color = if debug {
        stroke.color.with_saturation(0.1)
    } else {
        stroke.color
    };

    builder.build();

    for p in &builder.vertices {
        vertices.push([
            stroke.stroke_origin.x + p.x,
            stroke.stroke_origin.y + p.y,
            0.0,
        ]);
        let mut color = builder.default_color.to_linear().to_f32_array();

        color[3] = z_offset;
        colors.push(color);
    }

    (vertices, builder.indices, colors)
}
