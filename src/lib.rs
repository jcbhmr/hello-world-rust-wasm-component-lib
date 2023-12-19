wai_bindgen_rust::export!("hello-world-wasm-lib.wai");
pub struct HelloWorldWasmLib;
impl hello_world_wasm_lib::HelloWorldWasmLib for HelloWorldWasmLib {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}
