use bevy::{
    asset::Assets,
    color::Color,
    ecs::{
        component::Component,
        system::{ResMut, Single},
    },
    image::Image,
    log::info,
    render::{camera::Camera, render_resource::Extent3d},
    utils::default,
    window::Window,
};

use crate::retained_view::{RetainedTexture, RetainedView, ViewportTextureMaterial};

#[derive(Component, Default)]
pub struct TextureResizer {
    pub prev_width: u32,
    pub prev_height: u32,
}

pub fn resize_texture_system(
    mut retained_camera: Single<(&mut TextureResizer, &mut Camera)>,
    mut images: ResMut<Assets<Image>>,
    mut window: Single<&mut Window>,
    mut materials: ResMut<Assets<ViewportTextureMaterial>>,
    mut texture: ResMut<RetainedTexture>,
) {
    let height = window.physical_height();
    let width = window.physical_width();

    if retained_camera.0.prev_height == height && retained_camera.0.prev_width == width {
        return;
    }

    retained_camera.0.prev_height = height;
    retained_camera.0.prev_width = width;

    let image = images.get_mut(&texture.image_handle);

    match image {
        Some(image) => {
            if image.size().x != window.physical_width()
                || image.size().y != window.physical_height()
            {
                info!(
                    "Resizing image!: {} {}",
                    window.physical_width(),
                    window.physical_height()
                );

                image.resize(Extent3d {
                    width: window.physical_width(),
                    height: window.physical_height(),
                    ..default()
                });

                // Have to do this due to: https://github.com/bevyengine/bevy/issues/17350
                let _ = match materials.get_mut(&texture.material_handle) {
                    Some(_) => {
                        info!("Got material!");
                    }
                    None => (),
                };
            }
        }
        None => (),
    }
}
