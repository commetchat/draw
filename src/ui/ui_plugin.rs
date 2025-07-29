use bevy::app::{Plugin, Update};

use crate::ui::{
    ui_messages::{ReceivedUIMessage, SentUIMessage},
    ui_messages_queue::handle_queue,
};

pub struct AppUIPlugin;

impl Plugin for AppUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<ReceivedUIMessage>();
        app.add_event::<SentUIMessage>();
        app.add_systems(Update, handle_queue);
    }
}
