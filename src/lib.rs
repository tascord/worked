pub mod worker;
pub use ww_macro::worked;
pub use wasm_bindgen::prelude::*;
pub use bincode::{Decode, Encode};
pub use js_sys::Array;
pub use worker::WrappedWorker;

#[cfg(feature="example")]
mod example;