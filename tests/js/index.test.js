import test from "node:test";
import assert from "node:assert/strict";
import { bindings } from "@jcbhmr/hello-world-wasm-lib";
import autoBind from "auto-bind"
const { add } = autoBind(await bindings.hello_world_wasm_lib())

test("add(1, 2) == 3", async () => {
  assert.equal(add(1, 2), 3)
})
