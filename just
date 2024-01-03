#!/bin/bash
set -e

build()(
  if [[ $1 == '--release' ]]; then
    cargo build --target wasm32-wasi --release
    artifact=$(echo target/wasm32-wasi/release/*.wasm)
  else
    cargo build --target wasm32-wasi
    artifact=$(echo target/wasm32-wasi/debug/*.wasm)
  fi
  wasm-tools component new "$artifact" -o "$artifact.new" \
    --adapt ./wasi_snapshot_preview1.reactor.wasm
  mv "$artifact.new" "$artifact"
)

cd "$(dirname "${BASH_SOURCE[0]}")"; "$@"
