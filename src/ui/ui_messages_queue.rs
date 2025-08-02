use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

use bevy::{
    ecs::event::EventWriter,
    log::info,
};
use wasm_bindgen::prelude::wasm_bindgen;

use crate::ui::ui_messages::{ReceivedUIMessage, UIMessage};

static QUEUE: LazyLock<Mutex<VecDeque<UIMessage>>> = LazyLock::new(|| Mutex::new(VecDeque::new()));

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = gameUtils)]
    fn ui_message_callback(message: String);
}

pub fn send_ui_message(message: UIMessage) {
    match serde_json::to_string(&message) {
        Ok(str) => {
            ui_message_callback(str);
        }
        Err(_) => {
            info!("Failed to send UI message to front end");
        }
    }
}

pub fn handle_queue(mut events: EventWriter<ReceivedUIMessage>) {
    let mut a = QUEUE.lock().unwrap();

    loop {
        let item = VecDeque::pop_front(&mut a);
        match item {
            Some(event) => {
                info!("Writing UI Message: {:?}", event);
                events.write(ReceivedUIMessage { data: event });
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
