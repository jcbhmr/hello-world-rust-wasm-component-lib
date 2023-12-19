import test from "node:test";
import assert from "node:assert/strict";
import autoBind from "auto-bind"
import { bindings } from "@jcbhmr/hello-world-wasm-lib";
const { add, greet, greetMany, distanceBetween, perimeterOfCircle, areaOfCircle, multiLineLength } = autoBind(await bindings.exports())

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

test("perimeterOfCircle({{0,0},5}) ~= 10pi", () => {
  console.log(perimeterOfCircle({center:{x:0,y:0},radius:5}))
  console.log("10pi:", 10 * Math.PI)
})

test("areaOfCircle({{0,0},5}) ~= 25pi", () => {
  console.log(areaOfCircle({center:{x:0,y:0},radius:5}))
  console.log("25pi:", 25 * Math.PI)
})
