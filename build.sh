#!/bin/bash
set -e
mkdir -p out

if [[ $1 == '--release' ]]; then
  release_flag="--release"
  artifact=target/wasm32-wasi/release/hello_world_rust_wasm_component.wasm
else
  artifact=target/wasm32-wasi/debug/hello_world_rust_wasm_component.wasm
fi

if [[ ! -f out/wasi_snapshot_preview1.wasm ]]; then
  wget https://github.com/bytecodealliance/wasmtime/releases/download/dev/wasi_snapshot_preview1.reactor.wasm \
    -O out/wasi_snapshot_preview1.wasm
fi

cargo build --target wasm32-wasi $release_flag

wasm-tools component new "$artifact" \
  -o "out/$(basename "$artifact")" \
  --adapt ./out/wasi_snapshot_preview1.wasm

# wasm-tools component wit "out/$(basename "$artifact")"
