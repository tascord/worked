#!/bin/bash
clear
cargo build --package wasm-workers --target wasm32-unknown-unknown
wasm-bindgen --target web target/wasm32-unknown-unknown/debug/wasm_workers.wasm --out-dir dist