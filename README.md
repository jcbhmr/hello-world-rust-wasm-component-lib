## Installation

The easiest way to get started is by using one of these wrapper libraries that
already have all of the WASM host imports resolved.

For JavaScript users, you can use npm, [pnpm], [Yarn], or your other favorite npm package manager to install
[`hello-world-rust-wasm-component`] from npm.

```sh
npm install hello-world-rust-wasm-component
```

If you prefer to customize the Javascript ↔ WASM component bindings, you can use
[@bytecodealliance/jco] to generate your own bindings.

<!-- For Python users, you can use pip, [Poetry], or another PyPI package manager to
install [`hello-world-rust-wasm-component`] from PyPI.

```sh
pip install hello-world-rust-wasm-component
```

If you're using Rust, you can install [`hello-world-rust-wasm-component`] via Cargo.

```sh
cargo install hello-world-rust-wasm-component
``` -->

## Usage

You can use any [WASM host runtime for components]. You can find the API surface
in [`wit/host.wit`](wit/host.wit).

For language-specific binding documentation, check out
[`hello-world-rust-wasm-component.js`].

## Development

```sh
cargo install wasm-tools
```

```sh
./build.sh
./build.sh --release
```

<!-- prettier-ignore -->
✅ JavaScript: [jcbhmr/hello-world-rust-wasm-component/hello-world-rust-wasm-component.js](https://github.com/jcbhmr/hello-world-rust-wasm-component/tree/main/hello-world-rust-wasm-component.js) \
🟨 Python: [jcbhmr/hello-world-rust-wasm-component/hello-world-rust-wasm-component.py](https://github.com/jcbhmr/hello-world-rust-wasm-component/tree/main/hello-world-rust-wasm-component.py) \
🟨 Rust: [jcbhmr/hello-world-rust-wasm-component/hello-world-rust-wasm-component.rs](https://github.com/jcbhmr/hello-world-rust-wasm-component/tree/main/hello-world-rust-wasm-component.rs)

<!-- prettier-ignore-start -->
[WASM host runtime for components]: https://github.com/bytecodealliance/wit-bindgen#host-runtimes-for-components
<!-- prettier-ignore-end -->
