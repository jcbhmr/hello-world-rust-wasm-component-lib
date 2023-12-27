# "Hello world!" Rust WebAssembly library component

🟪 Rust library compiled to WASM and distributed as a [WASM component]

<table align=center><td>

```js
run();
//=> Hello Alan Turing!

console.log(getReport());
//=> {
//     bouncyCastles: 100,
//     funPercent: 0.9,
//     catCount: 8,
//     unicornNames: [ 'Fluffy', 'Marshmallow', 'Sparkles' ]
//   }

console.log(computeArea({ center: { x: 0, y: 0 }, radius: 6 }));
//=> 113.09733552923255

console.log(greetMany(["Alan Turing", "Ada Lovelace"]));
//=> Hello, Alan Turing!
//   Hello, Ada Lovelace!
```

</table>

🦀 Written in Rust \
[🟨 Usable in JavaScript](./hello-world-rust-wasm-component.js/) \
🌎 Runs on any [WebAssembly Component Runtime]

## Installation

![WebAssembly](https://img.shields.io/static/v1?style=for-the-badge&message=WebAssembly&color=654FF0&logo=WebAssembly&logoColor=FFFFFF&label=)
![JavaScript](https://img.shields.io/static/v1?style=for-the-badge&message=JavaScript&color=222222&logo=JavaScript&logoColor=F7DF1E&label=)

The easiest way to get started is by using one of these wrapper libraries that
already have all of the WASM host imports resolved.

For JavaScript users, you can use npm, [pnpm], [Yarn], or your other favorite npm package manager to install
[`hello-world-rust-wasm-component`] from npm.

```sh
npm install hello-world-rust-wasm-component
```

If you prefer to customize the Javascript ↔ WASM component bindings, you can use
[bytecodealliance/jco] to generate your own bindings.

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

![WebAssembly](https://img.shields.io/static/v1?style=for-the-badge&message=WebAssembly&color=654FF0&logo=WebAssembly&logoColor=FFFFFF&label=)
![JavaScript](https://img.shields.io/static/v1?style=for-the-badge&message=JavaScript&color=222222&logo=JavaScript&logoColor=F7DF1E&label=)

You can use any [WASM host runtime for components]. You can find the API surface
in [`wit/world.wit`](wit/world.wit).

For language-specific binding documentation, check out
[`hello-world-rust-wasm-component.js`].

## Development

![Rust](https://img.shields.io/static/v1?style=for-the-badge&message=Rust&color=000000&logo=Rust&logoColor=FFFFFF&label=)
![WebAssembly](https://img.shields.io/static/v1?style=for-the-badge&message=WebAssembly&color=654FF0&logo=WebAssembly&logoColor=FFFFFF&label=)

```sh
./build.sh
./build.sh --release
```

ℹ You'll need [`wasm-tools`] installed which you can get via `cargo install wasm-tools`.

<!-- prettier-ignore -->
✅ JavaScript: [jcbhmr/hello-world-rust-wasm-component/hello-world-rust-wasm-component.js](https://github.com/jcbhmr/hello-world-rust-wasm-component/tree/main/hello-world-rust-wasm-component.js) \
❌ Python \
❌ Rust

<!-- prettier-ignore-start -->
[WASM host runtime for components]: https://github.com/bytecodealliance/wit-bindgen#host-runtimes-for-components
[webassembly component runtime]: https://github.com/bytecodealliance/wit-bindgen#host-runtimes-for-components
[bytecodealliance/jco]: https://github.com/bytecodealliance/jco
[bytecodealliance/wasm-tools]: https://github.com/bytecodealliance/wasm-tools
[wasm component]: https://github.com/WebAssembly/component-model
[`wasm-tools`]: https://github.com/bytecodealliance/wasm-tools
<!-- prettier-ignore-end -->
