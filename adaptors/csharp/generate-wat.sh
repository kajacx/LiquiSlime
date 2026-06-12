#!/usr/bin/sh
set -e

# Run from the directory that you want the wasm files to be generated in

find . -type f -name '*.wasm' -print0 | while IFS= read -r -d '' f; do
  wasm2wat "$f" > "${f%.wasm}.wat" && echo "Processed file $f" || echo "Error on file $f"
done
