<div align="center">
    <h2>🤖 worked 🦀</h2>
    <div>
    <a href="https://crates.io/crates/worked"><img alt="Crates.io Version" src="https://img.shields.io/crates/v/worked?style=for-the-badge"></a>
    <a href="https://docs.rs/worked"><img alt="docs.rs" src="https://img.shields.io/docsrs/worked?style=for-the-badge"></a>    
    </div>
</div>

### Run wasm code in workers, without blocking
```rs
use worked::*;

#[wasm_bindgen(start)]
pub async fn start() {
    for i in 0..20 {
        factorial(
            i.clone(), // <-- Function input
            move |o| gloo::console::log!(&format!("{i}! = {o}")) // <-- Callback
        ).await; // <-- Await the spawning of the worker
    }
}

#[worked("/pkg/wasm_workers.js")] // <-- Absolute path to your main wasm_bindgen export
pub fn factorial(n: i64) -> i64 { // <-- Functions can only take one input
    f(n)
}

#[wasm_bindgen]
pub fn f(n: i64) -> i64 {
    match n {
        0 => 1,
        _ => n * f(n - 1),
    }
}
```