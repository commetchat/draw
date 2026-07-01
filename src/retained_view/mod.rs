use bevy::{
    asset::RenderAssetUsages,
    prelude::*,
    render::{
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, TextureDimension, TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin},
};

use crate::{
    BACKGROUND, RENDER_LAYER_BATCH_STROKES, RENDER_LAYER_RETAINED_IMAGE,
    retained_view::{
        camera_change_detection::{CameraMovementStatusEvent, camera_change_detection_system},
        copy_camera::{RetainedViewEvent, copy_camera_system},
        render_manager::{
            handle_render_events_system, render_loop, render_on_camera_movement_system,
        },
        resize_texture::{TextureResizer, resize_texture_system},
    },
    utils::DEBUG_DRAW,
};

pub struct CameraManagerPlugin;

#[derive(Component, Default, Debug)]
pub struct RetainedView {
    frames_until_disabled: i32,
}

#[derive(Resource)]

pub struct RetainedTexture {
    pub image_handle: Handle<Image>,
    pub material_handle: Handle<ViewportTextureMaterial>,
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct ViewportTextureMaterial {
    #[texture(0)]
    #[sampler(1)]
    color_texture: Option<Handle<Image>>,
}

#[derive(Component, Default)]
pub struct RetainedImagePlane {}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for ViewportTextureMaterial {
    fn fragment_shader() -> ShaderRef {
        const SHADER_ASSET_PATH: &str = "embedded://screenspace.wgsl";
        SHADER_ASSET_PATH.into()
    }
}

pub mod camera_change_detection;
pub mod copy_camera;
pub mod render_manager;
pub mod resize_texture;

pub struct RetainedViewPlugin;

impl Plugin for RetainedViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CameraMovementStatusEvent>();
        app.add_event::<RetainedViewEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(PostUpdate, copy_camera_system);
        app.add_systems(Update, resize_texture_system);
        app.add_systems(
            Update,
            (
                camera_change_detection_system,
                render_on_camera_movement_system,
                handle_render_events_system,
                render_loop,
            )
                .chain(),
        );
        app.add_plugins(Material2dPlugin::<ViewportTextureMaterial>::default());
    }
}
fn setup(
    mut images: ResMut<Assets<Image>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ViewportTextureMaterial>>,
) {
    let size = Extent3d {
        width: 1920,
        height: 1080,
        ..default()
    };
    let mut image = Image::new_fill(
        size.clone(),
        TextureDimension::D2,
        &[0, 0, 0, 0, 0, 0, 0, 0],
        TextureFormat::Rgba16Float,
        RenderAssetUsages::default(),
    );

    image.texture_descriptor.usage |= TextureUsages::RENDER_ATTACHMENT;

    let handle = images.add(image);

    let view_layer = RenderLayers::from_layers(&[RENDER_LAYER_BATCH_STROKES]);

    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            target: handle.clone().into(),
            clear_color: if DEBUG_DRAW {
                Color::linear_rgba(0.2, 0.1, 0.2, 0.0).into()
            } else {
                BACKGROUND.with_alpha(0.0).into()
            },
            ..default()
        },
        TextureResizer {
            prev_width: size.width,
            prev_height: size.height,
        },
        RetainedView {
            frames_until_disabled: 1,
        },
        view_layer.clone(),
    ));

    let material_handle = materials.add(ViewportTextureMaterial {
        color_texture: Some(handle.clone()),
    });

    let plane: Handle<Mesh> = meshes.add(Rectangle::from_size(Vec2 {
        x: 50000.0,
        y: 50000.0,
    }));

    commands.insert_resource(RetainedTexture {
        image_handle: handle,
        material_handle: material_handle.clone(),
    });

    commands.spawn((
        Mesh2d(plane),
        MeshMaterial2d(material_handle),
        RetainedImagePlane {},
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
        RenderLayers::from_layers(&[RENDER_LAYER_RETAINED_IMAGE]),
    ));
}
