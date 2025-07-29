use std::sync::{LazyLock, Mutex};

use bevy::{
    ecs::event::EventReader,
    log::info,
    math::{Vec2, VectorSpace},
};

use crate::{
    active_strokes::active_stroke::ActiveStrokeEvent,
    database::web_database::delete_stroke,
    stroke::StrokeMetadata,
    ui::ui_messages::{ReceivedUIMessage, UIMessage},
};
static UNDO_QUEUE: LazyLock<Mutex<Vec<(f64, u32)>>> = LazyLock::new(|| Mutex::new(Vec::new()));

pub fn undo_ui_system(mut events: EventReader<ReceivedUIMessage>) {
    let mut queue = UNDO_QUEUE.lock().unwrap();

    for event in events.read() {
        match &event.data {
            UIMessage::Undo => {
                let item = queue.pop();
                match item {
                    Some(item) => {
                        let meta = StrokeMetadata {
                            timestamp: item.0,
                            id_random: item.1,
                            owner: None,
                            origin: Vec2::ZERO,
                        };

                        let id = meta.get_id();

                        info!("Undoing: {}", id);

                        delete_stroke(id);
                    }
                    None => {
                        info!("Nothing left to undo!");
                    }
                }
            }
            _ => {}
        }
    }
}

pub fn store_undo_strokes_system(mut events: EventReader<ActiveStrokeEvent>) {
    let mut queue = UNDO_QUEUE.lock().unwrap();

    for event in events.read() {
        match event {
            ActiveStrokeEvent::StrokeFinished(stroke_finished_data) => {
                if stroke_finished_data.owner.is_some() {
                    continue;
                }

                queue.push((
                    stroke_finished_data.timestamp,
                    stroke_finished_data.id_random,
                ));

                info!("Appended to queue, current length: {}", queue.len());
            }
            _ => (),
        }
    }
}
