#!/bin/sh

cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings && cd wasm && cargo clippy -- --no-deps && cd ../meta && cargo clippy -- --no-deps
