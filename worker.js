import init, * as wasm from '/pkg/wasm_workers.js';

(async () => {
    await init();
    postMessage('ready');
    addEventListener('message', async event => {
        const {task_name, message} = JSON.parse(event.data);
        
        const task = wasm[task_name];
        if (!task) return console.error(`[Web Worker] Task '${task}' not found, is it exported with #[wasm_bindgen]?`);

        const result = await task(message);
        postMessage(result);
    });

})();