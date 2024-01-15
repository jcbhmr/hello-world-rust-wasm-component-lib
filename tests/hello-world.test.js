import test from "node:test";
import assert from "node:assert/strict";
import {
  cb,
  helloWorldRustWasmComponentLib,
} from "../target/js/test/hello_world_rust_wasm_component_lib.js";
import * as cbh from "./cbh.js";
import { inspect } from "node:util";

inspect.defaultOptions.showHidden = true;

test("setCb() and runCbWithResultOf()", () => {
  const f = (a) => {
    console.log({ a });
  };
  console.log("f:", f);
  const g = new cbh.PString(f);
  console.log("g:", g);
  const h = new cb.PString(g);
  console.log("h:", h);
  helloWorldRustWasmComponentLib.setCb(h);
  console.log("h (after set):", h);

  const i = () => {
    return "Hello world!";
  };
  console.log("i:", i);
  const j = new cbh.RString(i);
  console.log("j:", j);
  const k = new cb.RString(j);
  console.log("k:", k);
  console.log("h (before run):", h);
  helloWorldRustWasmComponentLib.runCbWithResultOf(k);
  console.log("k (after run):", k);
});
