use bevy::app::{Plugin, Update};

use crate::ui::{
    debug_ui::update_pending_stroke_count, ui_messages::{ReceivedUIMessage, SentUIMessage}, ui_messages_queue::handle_queue,
};

pub struct AppUIPlugin;

impl Plugin for AppUIPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_event::<ReceivedUIMessage>();
        app.add_event::<SentUIMessage>();
        app.add_systems(Update, handle_queue);
        app.add_systems(Update, update_pending_stroke_count);
    }
}
