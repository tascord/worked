use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use ww_macro::worked;

use crate::worker::WrappedWorker;

#[wasm_bindgen(start)]
pub async fn start() {
    console_error_panic_hook::set_once();
    greet("flora".to_string(), |o| gloo::console::log!(&o)).await;
}

#[worked("/pkg/wasm_workers.js")]
pub fn greet(name: String) -> String {
    format!("Hello, {name}!")
}
