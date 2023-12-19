import test from "node:test";
import assert from "node:assert/strict";
import autoBind from "auto-bind"
import { bindings } from "@jcbhmr/hello-world-wasm-lib";
const { add, greet, greetMany, distanceBetween } = autoBind(await bindings.exports())

test("add(1, 2) == 3", () => {
  assert.equal(add(1, 2), 3)
})

test("greet('Alan')", () => {
  console.log(greet('Alan'))
})

test("greetMany(['Alan', 'Ada'])", () => {
  console.log(greetMany(['Alan', 'Ada']))
})

test("distanceBetween({0,0}, {3,4}) == 5", () => {
  const a = { x: 0, y: 0 }
  const b = { x: 3, y: 4  }
  assert.equal(distanceBetween(a, b), 5)
})
