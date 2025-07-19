use bevy::prelude::*;

use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, PointerEvent};

use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

use crate::stylus_input::{PointerData, StylusEvent};

static ARRAY: LazyLock<Mutex<VecDeque<StylusEvent>>> =
    LazyLock::new(|| Mutex::new(VecDeque::new()));

pub struct WebInput;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

impl Plugin for WebInput {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_callbacks);
        app.add_systems(Update, handle_queue);
    }
}

fn handle_queue(mut events: EventWriter<StylusEvent>) {
    let mut a = ARRAY.lock().unwrap();
    loop {
        let item = VecDeque::pop_front(&mut a);
        match item {
            Some(event) => {
                events.write(event);
            }
            None => break,
        }
    }
}

fn setup_callbacks() {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let boxed: Box<dyn FnMut(PointerEvent)> = Box::new(move |e| {
        if (e.pointer_type() != "pen") {
            return;
        }

        let window = web_sys::window().expect("should have a window in this context");
        let document = window.document().expect("window should have a document");

        let binding = document
            .get_element_by_id("bevy-portal")
            .expect("should have #bevy-portal on the page");

        let canvas = binding
            .dyn_ref::<HtmlElement>()
            .expect("#bevy-portal be an `HtmlElement`");

        let rect = canvas.get_bounding_client_rect();

        let mut a = ARRAY.lock().unwrap();

        a.push_back(StylusEvent::PointerMove(PointerData {
            pressure: e.pressure(),
            position: Vec2 {
                x: (e.client_x() as f64 - rect.left()) as f32,
                y: (e.client_y() as f64 - rect.top()) as f32,
            },
        }));
    });
    let closure = Closure::wrap(boxed);

    document
        .get_element_by_id("bevy-portal")
        .expect("should have #bevy-portal on the page")
        .dyn_ref::<HtmlElement>()
        .expect("#bevy-portal be an `HtmlElement`")
        .set_onpointermove(Some(closure.as_ref().unchecked_ref()));

    closure.forget();
}
