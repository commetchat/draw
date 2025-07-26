use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

use bevy::{ecs::event::EventWriter, log::info};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::ui::ui_messages::UIMessage;

static QUEUE: LazyLock<Mutex<VecDeque<UIMessage>>> = LazyLock::new(|| Mutex::new(VecDeque::new()));

pub fn handle_queue(mut events: EventWriter<UIMessage>) {
    let mut a = QUEUE.lock().unwrap();

    loop {
        let item = VecDeque::pop_front(&mut a);
        match item {
            Some(event) => {
                info!("Writing UI Message: {:?}", event);
                events.write(event);
            }
            None => break,
        }
    }
}

#[wasm_bindgen]
pub fn queue_ui_message(json: String) {
    let message = serde_json::from_str::<UIMessage>(&json);
    match message {
        Ok(message) => {
            let mut queue = QUEUE.lock().unwrap();
            queue.push_back(message);
        }
        Err(_) => {
            info!("Invalid UI Message: {}", json);
        }
    }
}
