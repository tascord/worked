use crate::html_macros::*;
use crate::worker::WrappedWorker;
use dominator::{events::Click, Dom};
use futures_signals::signal::{Mutable, SignalExt};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{ops::Range, rc::Rc};
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

pub struct Model {
    status: Mutable<String>,
}

impl Model {
    async fn init() -> Rc<Model> {
        Rc::new(Model {
            status: Default::default(),
        })
    }

    fn render(self: Rc<Self>) -> Dom {
        div(|d| {
            d.children([
                h2(|d| {
                    d.text_signal(self.status.signal_cloned().map(|data| data))
                        .attr("id", "status")
                }),
                button(|d| {
                    d.text("Series Iteration").event({
                        let model = self.clone();
                        move |_: Click| {
                            // Run task in tokio thread (in a real use-case this would just be awaited properly)
                            let model = model.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                setup_worker(model, "series_iteration").await;
                            });
                        }
                    })
                }),
                button(|d| {
                    d.text("Paralell Iteration").event({
                        let model = self.clone();
                        move |_: Click| {
                            // Run task in tokio thread (in a real use-case this would just be awaited properly)
                            let model = model.clone();
                            wasm_bindgen_futures::spawn_local(async move {
                                setup_worker(model, "paralell_iteration").await;
                            });
                        }
                    })
                }),
            ])
        })
    }
}

#[wasm_bindgen]
pub async fn start_app() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    web_sys::console::log_1(&"Hit start".into());

    let app = Model::init().await;
    let body = dominator::body();

    dominator::append_dom(&body, Model::render(app));
    Ok(())
}

#[wasm_bindgen]
pub fn paralell_iteration() -> String {
    let now = js_sys::Date::now();
    let range: Range<u64> = 0..100_000_000;
    let result: u64 = range.into_par_iter().sum();
    format!(
        "[Paralell] Result: {} in {}ms",
        result,
        js_sys::Date::now() - now
    )
}

#[wasm_bindgen]
pub fn series_iteration() -> String {
    let now = js_sys::Date::now();
    let range: Range<u64> = 0..100_000_000;
    let result: u64 = range.into_iter().sum();
    format!(
        "[Series] Result: {} in {}ms",
        result,
        js_sys::Date::now() - now
    )
}

async fn setup_worker(model: Rc<Model>, task: &str) {
    model.status.set(String::new());

    let mut worker = WrappedWorker::new("/dist/worker.js").await;
    let result = worker
        .run_task(task, None::<()>)
        .await
        .deserialize::<String>();

    model.status.set(format!("{:?}", result.unwrap()));
    worker.kill();
}
