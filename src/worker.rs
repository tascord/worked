use std::sync::atomic::AtomicBool;

use bincode::{Decode, Encode};
use gloo_utils::{format::JsValueSerdeExt, window};
use js_sys::Array;
use serde_json::json;
use tokio::sync::mpsc::unbounded_channel;
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{MessageEvent, Worker, WorkerOptions, WorkerType};

pub type Callback = dyn FnMut(MessageEvent);
pub const WORKER_GLOB: &str = r#"
(async () => {
    const wasm = await import('%rel');
    await wasm.default();
    postMessage('ready');
    addEventListener('message', async event => {
        const {task_name, message} = JSON.parse(event.data);
        
        const task = wasm[task_name];
        if (!task) return console.error(`[Web Worker] Task '${task}' not found, is it exported with #[wasm_bindgen]?`);

        const result = await task(message);
        postMessage(result);
    });
})();
"#;

fn glob(rel: &str) -> String {
    web_sys::Url::create_object_url_with_blob(
        &web_sys::Blob::new_with_blob_sequence_and_options(
            &{
                let a = Array::new();
                a.push(
                    &WORKER_GLOB
                        .replace(
                            "%rel",
                            &format!(
                                "{}{}",
                                window().location().origin().unwrap().to_string(),
                                rel
                            ),
                        )
                        .to_string()
                        .into(),
                );
                a.into()
            },
            web_sys::BlobPropertyBag::new().type_("application/javascript"),
        )
        .unwrap(),
    )
    .unwrap()
}

pub struct WrappedWorker<I, O>
where
    I: Encode + Clone,
    O: Decode,
{
    worker: Worker,
    working: AtomicBool,
    _p: std::marker::PhantomData<(I, O)>,
}

impl<I, O> WrappedWorker<I, O>
where
    I: Encode + Clone,
    O: Decode,
{
    /// Create a new WrappedWorker
    ///
    /// Awaits worker ready (wasm_bindgen init)
    ///
    /// * `main_js` - Relative main _bg.js path
    pub async fn new(main_js: &str) -> WrappedWorker<I, O> {
        // Create and set options for the worker
        let mut worker_options = WorkerOptions::new();
        worker_options.type_(WorkerType::Module);

        let worker = Worker::new_with_options(&glob(main_js), &worker_options).unwrap();
        let (sender, mut receiver) = unbounded_channel::<()>();
        let handler = Closure::<Callback>::new(move |_: MessageEvent| {
            let _ = sender.send(());
        });

        worker.set_onmessage(Some(handler.as_ref().unchecked_ref()));
        handler.forget();
        receiver.recv().await;
        WrappedWorker {
            worker,
            working: AtomicBool::new(false),
            _p: std::marker::PhantomData,
        }
    }

    /// Run \#\[wasm_bindgen\] function.
    ///
    /// Awaits response as JsValue
    ///
    /// * `name` - Literal function name
    /// * `data` - Serializable data to pass to the function (one argument)
    /// * `callback` - Callback to handle the response
    pub fn run_task(&self, name: &str, data: I, callback: impl Fn(O) + 'static) {
        if self.working.load(std::sync::atomic::Ordering::Relaxed) {
            return;
        }

        self.working
            .store(true, std::sync::atomic::Ordering::Relaxed);

        let message = bincode::encode_to_vec(data.clone(), bincode::config::standard()).unwrap();

        let w = self.worker.clone();
        let handler = Closure::<Callback>::new(move |event: MessageEvent| {
            w.terminate();
            callback(
                bincode::decode_from_slice(
                    &Array::from(&event.data()).into_serde::<Vec<u8>>().unwrap(),
                    bincode::config::standard(),
                )
                .unwrap()
                .0,
            );
        });

        self.worker
            .set_onmessage(Some(handler.as_ref().unchecked_ref()));
        self.worker
            .post_message(
                &json!({ "task_name": name, "message": message })
                    .to_string()
                    .into(),
            )
            .expect("Failed to post message");

        handler.forget();
    }
}
