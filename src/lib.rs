
pub mod worker;

#[cfg(feature="example")]
pub mod html_macros;
#[cfg(feature="example")]
mod example;

#[allow(dead_code)]
pub use wasm_bindgen_rayon::init_thread_pool;