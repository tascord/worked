

import * as wasm from './wasm_workers.js';


async function run() {
    await wasm.default();
    await wasm.initThreadPool(navigator.hardwareConcurrency);

    // Tell the main thread we are ready
    postMessage('ready');

    addEventListener('message', async event => {
        const {task_name, data} = JSON.parse(event.data);

        const task = wasm[task_name];
        if (!task) return console.error(`[Web Worker] Task '${task}' not found, is it exported with #[wasm_bindgen]?`);

        let input = data;
        try { input = JSON.parse(data) } catch (e) {}

        const result = await task(input);
        postMessage(result);
    });

}

run()