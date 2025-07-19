use bevy::{
    app::{App, Plugin, Startup},
    asset::{Assets, RenderAssetUsages},
    color::{Color, ColorToComponents, ColorToPacked, Srgba},
    ecs::{
        event::EventWriter,
        system::{Commands, ResMut},
    },
    math::Vec2,
    render::mesh::{self, Mesh, Mesh2d},
    sprite::MeshMaterial2d,
    transform::components::Transform,
};
use bytes::Buf;

use crate::{
    BACKGROUND, CustomMaterial,
    file_reader::{self, read_string},
    line_builder::{LineBuilder, LineCapMode, LineJointMode},
    stroke::{Stroke, StrokeData, StrokeEvent, StrokeMetadata},
};
pub struct SaveLoad;

impl Plugin for SaveLoad {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut events: EventWriter<StrokeEvent>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let bytes = include_bytes!("../assets/canvas");
    let mut reader = bytes.reader();
    let len_header = file_reader::read_u32(&mut reader).unwrap();

    println!("Header: {}", len_header);

    file_reader::read_u32(&mut reader).unwrap();
    let _version = file_reader::read_u32(&mut reader).unwrap();
    let num_strokes = file_reader::read_u32(&mut reader).unwrap();
    println!("Num Strokes: {}", num_strokes);

    let mut indices = Vec::<u32>::new();
    let mut colors = Vec::<[f32; 4]>::new();
    let mut vertices = Vec::<[f32; 3]>::new();

    for i in 0..num_strokes {
        if i % 1000 == 0 {
            println!("Reading strokes: {}", i);
        }

        let stroke = read_stroke(None, &mut vertices, &mut indices, &mut colors, &mut reader);
        events.write(StrokeEvent::StrokeFinished(stroke));
    }

    println!("Reading remote strokes");
    let num_remote_ids = file_reader::read_u32(&mut reader).unwrap();
    for _ in 0..num_remote_ids {
        let id = file_reader::read_string(&mut reader).unwrap();

        let num_strokes = file_reader::read_u32(&mut reader).unwrap();

        for i in 0..num_strokes {
            if i % 1000 == 0 {
                println!("Reading strokes: {}", i);
            }

            let stroke = read_stroke(
                Some(id.clone()),
                &mut vertices,
                &mut indices,
                &mut colors,
                &mut reader,
            );
            events.write(StrokeEvent::StrokeFinished(stroke));
        }
    }

    println!("Num Vertices: {}", vertices.len());

    let mut line = Mesh::new(
        bevy::render::mesh::PrimitiveTopology::TriangleList,
        RenderAssetUsages::RENDER_WORLD,
    );

    line.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
    line.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);

    line.insert_indices(mesh::Indices::U32(indices));

    commands.spawn((
        // We use a marker component to identify the custom colored meshes
        // The `Handle<Mesh>` needs to be wrapped in a `Mesh2d` for 2D rendering
        Mesh2d(meshes.add(line)),
        MeshMaterial2d(materials.add(CustomMaterial {})),
        Transform::from_xyz(0., 0., 0.),
    ));

    // commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::default())),
    //     MeshMaterial2d(materials.add(CustomMaterial {
    //         color: GREEN_400.into(),
    //         color_texture: Some(asset_server.load("icon.png")),
    //     })),
    //     Transform::from_xyz(-100., 128., 0.).with_scale(Vec3::splat(256.)),
    // ));

    // commands.spawn((
    //     Mesh2d(meshes.add(Rectangle::default())),
    //     MeshMaterial2d(materials.add(CustomMaterial {
    //         color: GREEN_400.into(),
    //         color_texture: Some(asset_server.load("icon.png")),
    //     })),
    //     Transform::from_xyz(100., 128., 0.).with_scale(Vec3::splat(256.)),
    // ));
}

fn read_stroke(
    owner_id: Option<String>,
    vertices: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    colors: &mut Vec<[f32; 4]>,
    reader: &mut bytes::buf::Reader<&[u8]>,
) -> Stroke {
    let mut builder = LineBuilder::new();
    builder.end_cap_mode = LineCapMode::Round;
    builder.begin_cap_mode = LineCapMode::Round;
    builder.joint_mode = LineJointMode::Round;
    builder.width = 1.0;

    let id_random = file_reader::read_u32(reader).unwrap();
    let timestamp = file_reader::read_f64(reader).unwrap();

    let mut z_offset = timestamp;

    z_offset -= 1740000000.0;
    z_offset *= 0.00000001;

    let origin_x = file_reader::read_f32(reader).unwrap();
    let origin_y = file_reader::read_f32(reader).unwrap();
    let width = file_reader::read_f32(reader).unwrap();

    builder.width = width;

    let stroke_type = file_reader::read_u8(reader).unwrap();

    let mut col = BACKGROUND;

    if stroke_type == 1 {
        let r = file_reader::read_u8(reader).unwrap();
        let g = file_reader::read_u8(reader).unwrap();
        let b = file_reader::read_u8(reader).unwrap();

        col = Color::Srgba(Srgba::from_u8_array_no_alpha([r, g, b]));
    }

    let num_points = file_reader::read_u32(reader).unwrap();
    let mut points = Vec::new();
    points.reserve(usize::try_from(num_points).unwrap());

    for _ in 0..num_points {
        let x = file_reader::read_f32(reader).unwrap();
        let y = file_reader::read_f32(reader).unwrap();

        points.push(Vec2 { x: x, y: y });

        builder.points.push(Vec2 {
            x: (origin_x + x),
            y: -(origin_y + y),
        });
    }

    let has_pressure = file_reader::read_u8(reader).unwrap();
    let mut pressures: Option<Vec<f32>> = None;

    if has_pressure == 1 {
        let num_pressures = file_reader::read_u32(reader).unwrap();
        let mut p = Vec::new();
        p.reserve(usize::try_from(num_pressures).unwrap());
        for _ in 0..num_pressures {
            let pressure = file_reader::read_f32(reader).unwrap();
            p.push(pressure);
            builder.pressures.push(pressure);
        }

        pressures = Some(p);
    }

    // if stroke_type != 1 {
    //     return;
    // }

    let n_verts = vertices.len();

    builder.build();

    for p in &builder.vertices {
        vertices.push([p.x, p.y, 0.0]);
        let mut color = col.to_linear().to_f32_array(); // LinearRgba::from_u8_array_no_alpha().to_f32_array();

        color[3] = z_offset as f32;
        colors.push(color);
    }

    for i in &builder.indices {
        indices.push(*i + u32::try_from(n_verts).unwrap());
    }

    Stroke {
        metadata: StrokeMetadata {
            timestamp: timestamp,
            id_random: id_random,
            owner: owner_id,
            origin: Vec2 {
                x: origin_x,
                y: origin_y,
            },
        },
        data: StrokeData {
            points: points,
            pressures: pressures,
        },
    }
}
