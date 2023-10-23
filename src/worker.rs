use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::unbounded_channel;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{MessageEvent, Worker, WorkerOptions, WorkerType};

#[allow(dead_code)]
pub use wasm_bindgen_rayon::init_thread_pool;

pub type Callback = dyn FnMut(MessageEvent);

/// Generic worker.js file
///
/// This should be edited for more complex tasks,
/// but provides the boilerplate needed to execute
/// code exported by \#\[wasm_bindgen\]
pub const WORKER_FILE: &[u8] = include_bytes!("../worker.js");

#[derive(Serialize, Deserialize)]
struct WorkerTask<T: Serialize> {
    task_name: String,
    data: Option<T>,
}

pub struct WorkerOutput(Option<JsValue>);
impl WorkerOutput {
    pub fn value(&self) -> &Option<JsValue> {
        &self.0
    }
    pub fn deserialize<T: serde::de::DeserializeOwned>(
        &self,
    ) -> Result<T, serde_wasm_bindgen::Error> {
        serde_wasm_bindgen::from_value(
            self
            .0
            .as_ref()
            .unwrap_or(&JsValue::UNDEFINED)
            .clone()
        )
    }
}

impl<T: Serialize> WorkerTask<T> {
    pub fn new(task_name: String, data: Option<T>) -> Self {
        Self { task_name, data }
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }

    pub fn to_jsvalue(&self) -> JsValue {
        JsValue::from_str(&self.to_json())
    }
}

pub struct WrappedWorker {
    worker: Worker,
    pub open: bool,
}

impl WrappedWorker {
    /// Create a new WrappedWorker
    ///
    /// Awaits worker ready (wasm_bindgen init)
    ///
    /// * `path` - Path to worker.js file, see [`WORKER_FILE`]
    pub async fn new(path: &str) -> WrappedWorker {
        // Create and set options for the worker
        let mut worker_options = WorkerOptions::new();
        worker_options.type_(WorkerType::Module);

        // Spawn the worker
        let worker = Worker::new_with_options(path, &worker_options).unwrap();

        let (sender, mut receiver) = unbounded_channel::<()>();
        let handler = Closure::<Callback>::new(move |_: MessageEvent| {
            web_sys::console::log_1(&"[WebAssembly] Worker initialized.".into());
            let _ = sender.send(());
        });

        worker.set_onmessage(Some(handler.as_ref().unchecked_ref()));
        handler.forget();

        receiver.recv().await;
        WrappedWorker { worker, open: true }
    }

    /// Run \#\[wasm_bindgen\] function.
    ///
    /// Awaits response as JsValue
    ///
    /// * `name` - Literal function name
    /// * `data` - Serializable data to pass to the function (can be one argument)
    pub async fn run_task(&self, name: &str, data: Option<impl Serialize>) -> WorkerOutput {
        if !self.open {
            panic!("Worker has been killed.")
        }

        let task = WorkerTask::new(name.to_string(), data);
        let message = task.to_jsvalue();

        let (sender, mut receiver) = unbounded_channel::<JsValue>();
        let handler = Closure::<Callback>::new(move |event: MessageEvent| {
            let _ = sender.send(event.data()).unwrap();
        });

        let _ = &self
            .worker
            .set_onmessage(Some(handler.as_ref().unchecked_ref()));
        match &self.worker.post_message(&message) {
            Err(_) => {
                web_sys::console::log_1(&"[WebAssembly] Failed to post message to worker.".into());
            }
            Ok(_) => {}
        }

        handler.forget();
        WorkerOutput(receiver.recv().await)
    }

    /// Kill the worker
    ///
    /// Allows for safe initialization of a new worker as
    /// a new worker spins up a new thread pool
    pub fn kill(&mut self) {
        self.worker.terminate();
        self.open = false;
    }
}
