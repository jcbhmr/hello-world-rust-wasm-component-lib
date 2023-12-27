#!/bin/bash
set -e
mkdir -p out

if [[ $1 == '--release' ]]; then
  release_flag="--release"
  artifact=target/wasm32-wasi/release/hello_world_rust_wasm_component.wasm
else
  release_flag=""
  artifact=target/wasm32-wasi/debug/hello_world_rust_wasm_component.wasm
fi

cargo build --target wasm32-wasi $release_flag

wasm-tools component new "$artifact" \
  -o "out/$(basename "$artifact")" \
  --adapt ./wasi_snapshot_preview1.reactor.wasm

# wasm-tools component wit "out/$(basename "$artifact")"
