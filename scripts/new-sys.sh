#!/usr/bin/sh
set -e

# Run from parent folder
./scripts/new-build.sh

cd main-game
cargo run -p liquislime-macroquad
