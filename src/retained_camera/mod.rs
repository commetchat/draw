use bevy::{
    asset::RenderAssetUsages,
    color::palettes::css::GREEN,
    prelude::*,
    render::{
        mesh::PlaneMeshBuilder,
        render_resource::{
            AsBindGroup, Extent3d, ShaderRef, Texture, TextureDescriptor, TextureDimension,
            TextureFormat, TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin},
};

use crate::{
    RENDER_LAYER_BATCH_STROKES, RENDER_LAYER_RETAINED_IMAGE,
    retained_camera::{
        camera_manager::camera_render_toggle_system, copy_camera::copy_camera_system,
        resize_texture::resize_texture_system,
    },
};

pub struct CameraManagerPlugin;
pub mod camera_manager;

#[derive(Component, Default)]
pub struct RetainedCamera {
    prev_width: u32,
    prev_height: u32,
    disable_next_frame: bool,
}

#[derive(Resource)]

struct RetainedTexture {
    image_handle: Handle<Image>,
    material_handle: Handle<ViewportTextureMaterial>,
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

pub mod copy_camera;
pub mod resize_texture;

pub struct RetainedCameraPlugin;

impl Plugin for RetainedCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(PostUpdate, copy_camera_system);
        app.add_systems(PostUpdate, resize_texture_system);
        app.add_systems(Update, camera_render_toggle_system);
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
        size,
        TextureDimension::D2,
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        TextureFormat::Rgba32Float,
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
            clear_color: Color::linear_rgb(1.0, 0.0, 1.0).into(),
            ..default()
        },
        RetainedCamera {
            prev_height: 0,
            prev_width: 0,
            disable_next_frame: true,
        },
        view_layer.clone(),
    ));

    let material_handle = materials.add(ViewportTextureMaterial {
        color_texture: Some(handle.clone()),
    });

    let plane: Handle<Mesh> = meshes.add(Rectangle::from_size(Vec2 {
        x: 5000.0,
        y: 5000.0,
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
