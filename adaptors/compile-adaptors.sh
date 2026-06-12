#!/usr/bin/sh
set -e

# Call from this directory

cd csharp
./compile-csharp.sh
cd ..

cd rust
./compile-rust.sh
cd ..
