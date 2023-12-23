import test from "node:test"
import assert from "node:assert"
import * as hello_world_rust_wasm_component from "./dist/hello_world_rust_wasm_component.js"

console.log(hello_world_rust_wasm_component)

test("greet() works", () => {
  console.log(hello_world_rust_wasm_component.greet("Alan Turing"))
})

test("greetMany() works", () => {
  console.log(hello_world_rust_wasm_component.greetMany(["Alan Turing", "Ada Lovelace"]))
})

test("run() works", () => {
  hello_world_rust_wasm_component.run()
})

test("getReport() works", () => {
  console.log(hello_world_rust_wasm_component.getReport())
})
