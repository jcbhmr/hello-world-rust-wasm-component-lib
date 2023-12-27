# "Hello world!" Rust WebAssembly library component for JavaScript

🔗 JavaScript bindings for [jcbhmr/hello-world-rust-wasm-component]

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

## Installation

![npm](https://img.shields.io/static/v1?style=for-the-badge&message=npm&color=CB3837&logo=npm&logoColor=FFFFFF&label=)
![Yarn](https://img.shields.io/static/v1?style=for-the-badge&message=Yarn&color=2C8EBB&logo=Yarn&logoColor=FFFFFF&label=)
![pnpm](https://img.shields.io/static/v1?style=for-the-badge&message=pnpm&color=222222&logo=pnpm&logoColor=F69220&label=)
![Deno](https://img.shields.io/static/v1?style=for-the-badge&message=Deno&color=000000&logo=Deno&logoColor=FFFFFF&label=)
![jsDelivr](https://img.shields.io/static/v1?style=for-the-badge&message=jsDelivr&color=E84D3D&logo=jsDelivr&logoColor=FFFFFF&label=)

```sh
npm install hello-world-rust-wasm-component
```

```js
import {} from "npm:hello-world-rust-wasm-component";
```

```html
<script type="module">
  import {} from "https://esm.run/hello-world-rust-wasm-component";
</script>
```

## Usage

![Node.js](https://img.shields.io/static/v1?style=for-the-badge&message=Node.js&color=339933&logo=Node.js&logoColor=FFFFFF&label=)
![Deno](https://img.shields.io/static/v1?style=for-the-badge&message=Deno&color=000000&logo=Deno&logoColor=FFFFFF&label=)
![Bun](https://img.shields.io/static/v1?style=for-the-badge&message=Bun&color=000000&logo=Bun&logoColor=FFFFFF&label=)
![Browser](https://img.shields.io/static/v1?style=for-the-badge&message=Browser&color=4285F4&logo=Google+Chrome&logoColor=FFFFFF&label=)

```js
import {
  run,
  greetMany,
  getReport,
  computeArea,
} from "hello-world-rust-wasm-component";

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

<!-- prettier-ignore-start -->
[jcbhmr/hello-world-rust-wasm-component]: https://github.com/jcbhmr/hello-world-rust-wasm-component
<!-- prettier-ignore-end -->
