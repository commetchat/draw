pub struct WebInput;
use bevy::prelude::*;

use js_sys::{Array, Date};
use wasm_bindgen::prelude::*;
use web_sys::{HtmlElement, PointerEvent};

use std::{
    collections::VecDeque,
    sync::{LazyLock, Mutex},
};

static ARRAY: LazyLock<Mutex<VecDeque<StylusEvent>>> =
    LazyLock::new(|| Mutex::new(VecDeque::new()));

struct PointerMoveData {
    pressure: f32,
}

enum StylusEvent {
    PointerMove(PointerMoveData),
}

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

fn handle_queue(mut commands: Commands) {
    console_log!("Updating frame");

    let mut a = ARRAY.lock().unwrap();
    loop {
        let item = VecDeque::pop_front(&mut a);
        match item {
            Some(event) => match event {
                StylusEvent::PointerMove(pointer_move_data) => {
                    console_log!("Got stylus event: {}", pointer_move_data.pressure)
                }
            },
            None => break,
        }
    }
}

fn setup_callbacks(mut commands: Commands) {
    let window = web_sys::window().expect("should have a window in this context");
    let document = window.document().expect("window should have a document");

    let mut clicks = 0;

    let boxed: Box<dyn FnMut(PointerEvent)> = Box::new(move |e| {
        let mut a = ARRAY.lock().unwrap();

        a.push_back(StylusEvent::PointerMove(PointerMoveData {
            pressure: e.pressure(),
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
