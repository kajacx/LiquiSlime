#!/usr/bin/sh
set -e

# Call from this directory

cd SlimeDragger
find . -name "*.wasm" -type f -delete
find . -name "*.wat" -type f -delete
# find . -name "*.wit" -type f -delete
# dotnet clean -r wasi-wasm
# dotnet publish -r wasi-wasm
dotnet build
# wasm2wat bin/Debug/net10.0/wasiconsole.wasm > bin/Debug/net10.0/wasiconsole.wat
# wasm-tools component wit bin/Release/net10.0/wasi-wasm/publish/wasiconsole.wasm > wasiconsole.wit
# wasm-tools component wit bin/Debug/net10.0/wasi-wasm/publish/adder.wasm > adder.txt
cd ..

# ./generate-wat.sh
