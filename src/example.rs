use console_error_panic_hook;
use wasm_bindgen::prelude::*;
use ww_macro::worked;

use crate::worker::WrappedWorker;

#[wasm_bindgen(start)]
pub async fn start() {
    console_error_panic_hook::set_once();
    for i in 0..20 {
        factorial(i.clone(), move |o| gloo::console::log!(&format!("{i}! = {o}"))).await;
    }
}

#[worked("/pkg/worked.js")]
pub fn factorial(n: i64) -> i64 {
    f(n)
}

#[wasm_bindgen]
pub fn f(n: i64) -> i64 {
    match n {
        0 => 1,
        _ => n * f(n - 1),
    }
}