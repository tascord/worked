#!/bin/bash
clear
cargo build --package wasm-workers --target wasm32-unknown-unknown -Z build-std=std,panic_abort
wasm-bindgen --target web target/wasm32-unknown-unknown/debug/wasm_workers.wasm --out-dir dist