#!/usr/bin/sh
set -e

# Call from this directory

cd SlimeDragger
dotnet build
wasm2wat bin/Debug/net10.0/wasiconsole.wasm > bin/Debug/net10.0/wasiconsole.wat
cd ..
