import test from "node:test";
import assert from "node:assert";
import * as hello_world_rust_wasm_lib_component from "./dist/hello_world_rust_wasm_lib_component.js";

console.log(hello_world_rust_wasm_lib_component);

test("greet() works", () => {
  console.log(hello_world_rust_wasm_lib_component.greet("Alan Turing"));
});

test("greetMany() works", () => {
  console.log(
    hello_world_rust_wasm_lib_component.greetMany(["Alan Turing", "Ada Lovelace"]),
  );
});

test("run() works", () => {
  hello_world_rust_wasm_lib_component.run();
});

test("getReport() works", () => {
  console.log(hello_world_rust_wasm_lib_component.getReport());
});

test("computeArea() works", () => {
  console.log(
    hello_world_rust_wasm_lib_component.computeArea({
      center: { x: 0, y: 0 },
      radius: 6,
    }),
  );
  console.log("from JS", Math.PI * 6 * 6);
});
