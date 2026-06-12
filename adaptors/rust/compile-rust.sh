#!/usr/bin/sh
set -e

# Call from this directory

cd slime-clicker
cargo build --target=wasm32-wasip1
cd ..