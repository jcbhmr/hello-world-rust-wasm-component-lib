use std::collections::HashMap;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};

bindgen!();
use jcbhmr::hello_world_rust_wasm_component_lib::*;

struct MyState {
    p_string: HashMap<u32, Box<dyn Fn(&str)>>,
    r_string: HashMap<u32, Box<dyn Fn() -> String>>,
}
impl MyState {
    fn new() -> Self {
        Self {
            p_string: HashMap::new(),
            r_string: HashMap::new(),
        }
    }
}
impl cbh::HostPString for MyState {
    fn call(&mut self, self_: Resource<cbh::PString>, a: String) -> wasmtime::Result<()> {
        let f = self.p_string.get(&self_.rep()).unwrap();
        f(a.as_str());
        Ok(())
    }
    fn drop(&mut self, self_: Resource<cbh::PString>) -> wasmtime::Result<()> {
        self.p_string.remove(&self_.rep());
        Ok(())
    }
}
impl cbh::HostRString for MyState {
    fn call(&mut self, self_: Resource<cbh::RString>) -> wasmtime::Result<String> {
        let f = self.r_string.get(&self_.rep()).unwrap();
        Ok(f())
    }
    fn drop(&mut self, self_: Resource<cbh::RString>) -> wasmtime::Result<()> {
        self.r_string.remove(&self_.rep());
        Ok(())
    }
}
impl cbh::Host for MyState {}

#[test]
fn it_works() {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let component = Component::from_file(
        &engine,
        "./target/wasm32-unknown-unknown/debug/hello_world_rust_wasm_component_lib.wasm",
    )
    .unwrap();

    let mut linker = Linker::new(&engine);
    HelloWorldRustWasmComponentLibWorld::add_to_linker(&mut linker, |state: &mut MyState| state)
        .unwrap();

    let mut store = Store::new(&engine, MyState::new());
    let (bindings, _) =
        HelloWorldRustWasmComponentLibWorld::instantiate(&mut store, &component, &linker).unwrap();

    // Note that the `demo` method returns a `&Demo` through which we can
    // run the methods on that interface.
    let cb = bindings.jcbhmr_hello_world_rust_wasm_component_lib_cb();
    let cb_p_string = cb.p_string();
    let cb_r_string = cb.r_string();
    let hello_world_rust_wasm_component_lib =
        bindings.jcbhmr_hello_world_rust_wasm_component_lib_hello_world_rust_wasm_component_lib();

    let my_cb: Box<dyn Fn(&str)> = Box::new(|a: &str| println!("my_cb recieved: a={}", a));
    store.data_mut().p_string.insert(100, my_cb);
    let my_cb = cb_p_string
        .call_constructor(&mut store, Resource::new_own(100))
        .unwrap();
    hello_world_rust_wasm_component_lib
        .call_set_cb(&mut store, my_cb)
        .unwrap();

    let my_cb2: Box<dyn Fn() -> String> = Box::new(|| "Hello from my_cb2!".to_string());
    store.data_mut().r_string.insert(200, my_cb2);
    let my_cb2 = cb_r_string
        .call_constructor(&mut store, Resource::new_own(200))
        .unwrap();
    hello_world_rust_wasm_component_lib
        .call_run_cb_with_result_of(&mut store, my_cb2)
        .unwrap();
}
