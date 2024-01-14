import test from "node:test";
import assert from "node:assert/strict";
import * as hello_world_rust_wasm_component_lib from "../target/js/test/hello_world_rust_wasm_component_lib.js";
import {
  forEach,
  forEachCb,
  myBox
} from "../target/js/test/hello_world_rust_wasm_component_lib.js";
import { CbStringU32ListString, CbU32U32ListU32 } from "./for-each-cb-host.js";

console.log(hello_world_rust_wasm_component_lib);

test.skip("forEach.forEachString", () => {
  const cb = forEachCb.CbStringU32ListString.wrap(
    new CbStringU32ListString((a, b, c) => {
      console.log(a, b, c);
    })
  );
  forEach.forEachString(["a", "b", "c"], cb);
});

test.skip("forEach.forEachU32", () => {
  const cb = forEachCb.CbU32U32ListU32.wrap(
    new CbU32U32ListU32((a, b, c) => {
      console.log(a, b, c);
    })
  );
  forEach.forEachU32([1, 2, 3], cb);
});

test("my box", () => {
  console.log(myBox.globalValue());
  myBox.setGlobal(new myBox.MyBox(42));
  console.log(myBox.globalValue());
})
