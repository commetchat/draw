use std::ops::Deref;

use bevy::{
    ecs::{
        component::Component,
        entity::Entity,
        event::EventReader,
        observer::Trigger,
        system::{Commands, Single},
    },
    log::info,
    math::Vec2,
    render::view::screenshot::{Screenshot, ScreenshotCaptured},
    window::Window,
};

use crate::{
    stylus_input::StylusEvent,
    tools::tools_plugin::ActiveTool,
    ui::{
        ui_messages::{Color, ReceivedUIMessage, UIMessage},
        ui_messages_queue::send_ui_message,
    },
};

#[derive(Component, Default, Debug)]
pub struct ToolColorPicker {}

pub fn color_picker_system(
    mut events: EventReader<StylusEvent>,
    window: Single<&mut Window>,
    tool: Single<(&mut ToolColorPicker, &mut ActiveTool)>,
    mut commands: Commands,
) {
    for event in events.read() {
        let event = match event {
            StylusEvent::PointerUp(pointer_data) => pointer_data,
            _ => {
                return;
            }
        };

        let scale = window.resolution.base_scale_factor();

        let window_size = Vec2 {
            x: window.resolution.width(),
            y: window.resolution.height(),
        };

        let pos = event.position * scale;
        let pos = pos / window_size;

        info!("Picking color at: {}", pos);

        commands
            .spawn(Screenshot::primary_window())
            .observe(handle_image(pos));
    }
}

pub fn handle_image(position: Vec2) -> impl FnMut(Trigger<ScreenshotCaptured>) {
    move |trigger| {
        let img = trigger.event().deref().clone();

        let x = img.width() as f32 * position.x;
        let y = img.height() as f32 * position.y;

        match img.get_color_at(x as u32, y as u32) {
            Ok(color) => {
                info!("Got color: {:?}", color);
                let col = color.to_srgba();
                send_ui_message(UIMessage::SetColor(Color {
                    r: col.red,
                    g: col.green,
                    b: col.blue,
                }));
            }
            Err(_) => {
                info!("Failed to get color!");
            }
        }
    }
}

pub fn color_picker_ui_system(
    picker: Single<(Entity, &mut ToolColorPicker)>,
    mut events: EventReader<ReceivedUIMessage>,
    mut commands: Commands,
) {
    for event in events.read() {
        match &event.data {
            UIMessage::SetTool(tool) => match tool {
                crate::ui::ui_messages::Tool::ColorPicker => {
                    info!("Received set tool event!, inserting active tool to color picker");
                    commands.entity(picker.0).insert_if_new(ActiveTool {});
                }
                _ => {
                    info!("Received set tool event!, removing active tool from color picker");
                    commands.entity(picker.0).remove::<ActiveTool>();
                }
            },
            _ => (),
        }
    }
}
