//! A shader and a material that uses it.

use std::{io::Read, ops::ControlFlow};

#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::YELLOW,
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{self},
        render_resource::{AsBindGroup, ShaderRef},
    },
    sprite::{Material2d, Material2dPlugin},
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use binreader::{BinReader, OwnableBinReader, RandomAccessBinReader};
use iyes_perf_ui::{PerfUiPlugin, prelude::PerfUiDefaultEntries};

use crate::{
    camera_controller::{CameraControllerPlugin, TouchCameraController},
    lerp_transform::{LerpTransformPlugin, TargetTransform},
    line_builder::{LineBuilder, LineCapMode, LineJointMode},
    stylus_drawer::StylusDrawer,
    stylus_input::StylusInput,
    web_input::WebInput,
};

pub mod camera_controller;
pub mod file_reader;
pub mod lerp_transform;
pub mod line_builder;
pub mod stylus_drawer;
pub mod stylus_input;
pub mod web_input;
use bytes::Buf;

/// This example uses a shader source file from the assets subdirectory

const BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.1);

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Test".to_string(),
            canvas: Some("#bevy-portal".to_string()),
            fit_canvas_to_parent: true,
            ..default()
        }),
        ..default()
    }))
    .insert_resource(ClearColor(BACKGROUND))
    .add_plugins(PerfUiPlugin)
    .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin::default())
    .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)
    .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin)
    .add_plugins(bevy::render::diagnostic::RenderDiagnosticsPlugin)
    .add_plugins(EmbeddedAssetPlugin::default())
    .add_plugins(Material2dPlugin::<CustomMaterial>::default())
    .add_plugins(StylusInput)
    .add_plugins(StylusDrawer)
    .add_plugins(LerpTransformPlugin)
    .add_plugins(CameraControllerPlugin)
    .add_systems(Startup, setup);

    #[cfg(target_arch = "wasm32")]
    app.add_plugins(WebInput);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(Wireframe2dPlugin::default());

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_wireframe);

    app.run();
}

#[cfg(not(target_arch = "wasm32"))]
fn toggle_wireframe(
    mut wireframe_config: ResMut<Wireframe2dConfig>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        wireframe_config.global = !wireframe_config.global;
    }
}

// Setup a simple 2d scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera2d,
        TargetTransform::default(),
        TouchCameraController::default(),
    ));

    let bytes = include_bytes!("../assets/canvas");
    let mut reader = bytes.reader();
    let len_header = file_reader::read_u32(&mut reader).unwrap();

    println!("Header: {}", len_header);

    file_reader::read_u32(&mut reader).unwrap();
    let version = file_reader::read_u32(&mut reader).unwrap();
    let num_strokes = file_reader::read_u32(&mut reader).unwrap();
    println!("Num Strokes: {}", num_strokes);

    let mut indices = Vec::<u32>::new();
    let mut colors = Vec::<[f32; 4]>::new();
    let mut vertices = Vec::<[f32; 3]>::new();

    for i in 0..num_strokes {
        if i % 1000 == 0 {
            println!("Reading strokes: {}", i);
        }

        read_stroke(&mut vertices, &mut indices, &mut colors, &mut reader);
    }

    println!("Reading remote strokes");
    let num_remote_ids = file_reader::read_u32(&mut reader).unwrap();
    for i in 0..num_remote_ids {
        let str_len = file_reader::read_u32(&mut reader).unwrap();
        for x in 0..str_len {
            file_reader::read_u8(&mut reader).unwrap();
        }

        let num_strokes = file_reader::read_u32(&mut reader).unwrap();

        for i in 0..num_strokes {
            if i % 1000 == 0 {
                println!("Reading strokes: {}", i);
            }

            read_stroke(&mut vertices, &mut indices, &mut colors, &mut reader);
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

    let color = BACKGROUND;

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

    commands.spawn(PerfUiDefaultEntries::default());
}

fn read_stroke(
    vertices: &mut Vec<[f32; 3]>,
    indices: &mut Vec<u32>,
    colors: &mut Vec<[f32; 4]>,
    reader: &mut bytes::buf::Reader<&[u8]>,
) {
    let mut builder = LineBuilder::new();
    builder.end_cap_mode = LineCapMode::Round;
    builder.begin_cap_mode = LineCapMode::Round;
    builder.joint_mode = LineJointMode::Round;
    builder.width = 1.0;

    let id_random = file_reader::read_u32(reader).unwrap();
    let mut timestamp = file_reader::read_f64(reader).unwrap();

    timestamp -= 1740000000.0;
    timestamp *= 0.00000001;

    println!("timestamp: {}", timestamp);

    let origin_x = file_reader::read_f32(reader).unwrap();
    let origin_y = file_reader::read_f32(reader).unwrap();
    let width = file_reader::read_f32(reader).unwrap();

    builder.width = width;

    let stroke_type = file_reader::read_u8(reader).unwrap();
    let mut r: u8 = 255;
    let mut g: u8 = 255;
    let mut b: u8 = 255;

    let mut col = BACKGROUND;

    if stroke_type == 1 {
        r = file_reader::read_u8(reader).unwrap();
        g = file_reader::read_u8(reader).unwrap();
        b = file_reader::read_u8(reader).unwrap();

        col = Color::Srgba(Srgba::from_u8_array_no_alpha([r, g, b]));
    }

    let num_points = file_reader::read_u32(reader).unwrap();

    for i in 0..num_points {
        let x = file_reader::read_f32(reader).unwrap();
        let y = file_reader::read_f32(reader).unwrap();

        builder.points.push(Vec2 {
            x: (origin_x + x),
            y: -(origin_y + y),
        });
    }

    let has_pressure = file_reader::read_u8(reader).unwrap();

    if has_pressure == 1 {
        let num_pressures = file_reader::read_u32(reader).unwrap();
        for i in 0..num_pressures {
            let pressure = file_reader::read_f32(reader).unwrap();
            builder.pressures.push(pressure);
        }
    }

    // if stroke_type != 1 {
    //     return;
    // }

    let n_verts = vertices.len();

    builder.build();

    let mut v_pos: Vec<[f32; 3]> = vec![];
    for p in &builder.vertices {
        vertices.push([p.x, p.y, 0.0]);
        let mut color = col.to_linear().to_f32_array(); // LinearRgba::from_u8_array_no_alpha().to_f32_array();

        color[3] = timestamp as f32;
        colors.push(color);
    }

    for i in &builder.indices {
        indices.push(*i + u32::try_from(n_verts).unwrap());
    }
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        const SHADER_ASSET_PATH: &str = "embedded://material.wgsl";
        SHADER_ASSET_PATH.into()
    }
}
