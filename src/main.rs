//! A shader and a material that uses it.

#[cfg(not(target_arch = "wasm32"))]
use bevy::sprite::{Wireframe2dConfig, Wireframe2dPlugin};

use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    reflect::TypePath,
    render::{
        mesh::{self},
        render_resource::{AsBindGroup, ShaderRef},
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin},
    ui::UiPlugin,
    window::WindowResolution,
};
use bevy_embedded_assets::EmbeddedAssetPlugin;
use iyes_perf_ui::{
    PerfUiPlugin,
    entries::{
        PerfUiFixedTimeEntries, PerfUiFramerateEntries, PerfUiSystemEntries, PerfUiWindowEntries,
    },
    prelude::{PerfUiDefaultEntries, PerfUiEntryEntityCount, PerfUiEntryFPS, PerfUiEntryFrameTime},
};
use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(target_arch = "wasm32")]
use crate::web_input::WebInput;

use crate::{
    active_strokes::active_strokes_plugin::ActiveStrokesPlugin,
    camera_controller::{CameraControllerPlugin, TouchCameraController},
    chunks::{ChunkController, ChunksPlugin},
    database::{Database, web_database::init_web_database},
    lerp_transform::{LerpTransformPlugin, TargetTransform},
    networking::networking_plugin::NetworkingPlugin,
    retained_view::{
        RetainedViewPlugin, camera_change_detection::CameraChangeDetector,
        copy_camera::TargetCamera,
    },
    stroke::Strokes,
    stylus_drawer::StylusDrawer,
    stylus_input::StylusInput,
    tools::tools_plugin::ToolsPlugin,
    ui::ui_plugin::AppUIPlugin,
};

pub mod camera_controller;

pub mod active_strokes;
pub mod chunks;
pub mod database;
pub mod file_reader;
pub mod lerp_transform;
pub mod line_builder;
pub mod load_file;
pub mod mesh_conversion;
pub mod networking;
pub mod retained_view;
pub mod stroke;
pub mod stylus_drawer;
pub mod stylus_input;
pub mod tools;
pub mod ui;
pub mod utils;
pub mod web_input;

/// This example uses a shader source file from the assets subdirectory

pub const BACKGROUND: Color = Color::srgb(0.1, 0.1, 0.1);

const RENDER_LAYER_DEFAULT: usize = 0;
const RENDER_LAYER_BATCH_STROKES: usize = 1;
const RENDER_LAYER_RETAINED_IMAGE: usize = 2;
const RENDER_LAYER_ACTIVE_STROKES: usize = 3;
const RENDER_LAYER_HUD: usize = 4;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Test".to_string(),
            resolution: WindowResolution::new(500.0, 500.0).with_scale_factor_override(1.0),
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
    .add_plugins(ChunksPlugin)
    .add_plugins(Strokes)
    .add_plugins(StylusInput)
    // .add_plugins(StylusDrawer)
    .add_plugins(ToolsPlugin)
    .add_plugins(ActiveStrokesPlugin)
    .add_plugins(AppUIPlugin)
    .add_plugins(Database)
    .add_plugins(RetainedViewPlugin)
    .add_plugins(LerpTransformPlugin)
    .add_plugins(CameraControllerPlugin)
    .add_plugins(NetworkingPlugin)
    .add_systems(Startup, setup);

    #[cfg(target_arch = "wasm32")]
    app.add_plugins(WebInput);

    #[cfg(not(target_arch = "wasm32"))]
    app.add_plugins(Wireframe2dPlugin::default());

    #[cfg(not(target_arch = "wasm32"))]
    app.add_systems(Update, toggle_wireframe);

    app.run();

    init_web_database();
}

// Import the `window.alert` function from the Web.
#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// Export a `greet` function from Rust to JavaScript, that alerts a
// hello message.
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
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
fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        RenderLayers::from_layers(&[
            RENDER_LAYER_DEFAULT,
            RENDER_LAYER_BATCH_STROKES,
            RENDER_LAYER_HUD,
            RENDER_LAYER_ACTIVE_STROKES,
            RENDER_LAYER_RETAINED_IMAGE,
        ]),
        TargetCamera {},
        CameraChangeDetector::default(),
        TargetTransform::default(),
        ChunkController::default(),
        TouchCameraController::default(),
    ));

    commands.spawn((
        PerfUiFramerateEntries::default(),
        PerfUiEntryEntityCount::default(),
        RenderLayers::layer(RENDER_LAYER_HUD),
    ));
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
