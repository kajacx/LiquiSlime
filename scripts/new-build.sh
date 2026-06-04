#!/usr/bin/sh
set -e

cd adaptors/rust/slime-clicker
# cargo build --target=wasm32-unknown-unknown
cargo build --target=wasm32-wasip1
