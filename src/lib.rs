cargo_component_bindings::generate!();

use std::borrow::Borrow;
use std::cell::{OnceCell, RefCell};
use std::sync::Arc;

use bindings::exports::jcbhmr::hello_world_rust_wasm_component_lib::my_box::GuestMyBox;
use bindings::exports::jcbhmr::hello_world_rust_wasm_component_lib::*;
use bindings::jcbhmr::hello_world_rust_wasm_component_lib::*;
use cargo_component_bindings::rt::Resource;
use once_cell::unsync::Lazy;
use send_wrapper::SendWrapper;

pub struct Component;
impl hello_world::Guest for Component {
    fn greet(name: String) -> String {
        format!("Hello, {}!", name)
    }
    fn say_hello(name: String) {
        io::println(format!("Hello, {}!", name).as_str());
    }
}

pub struct CbStringU32ListString(Box<dyn Fn(&str, u32, &[String])>);
impl for_each_cb::GuestCbStringU32ListString for CbStringU32ListString {
    fn wrap(cb: for_each_cb_host::CbStringU32ListString) -> for_each_cb::OwnCbStringU32ListString {
        Resource::new(CbStringU32ListString(Box::new(move |a, b, c| {
            cb.run(a, b, c);
        })))
    }
    fn run(&self, a: String, b: u32, c: Vec<String>) {
        (self.0)(a.as_str(), b, c.as_slice());
    }
}

pub struct CbU32U32ListU32(Box<dyn Fn(u32, u32, &[u32])>);
impl for_each_cb::GuestCbU32U32ListU32 for CbU32U32ListU32 {
    fn wrap(cb: for_each_cb_host::CbU32U32ListU32) -> for_each_cb::OwnCbU32U32ListU32 {
        for_each_cb::OwnCbU32U32ListU32::new(CbU32U32ListU32(Box::new(move |a, b, c| {
            cb.run(a, b, c);
        })))
    }
    fn run(&self, a: u32, b: u32, c: Vec<u32>) {
        (self.0)(a, b, c.as_slice());
    }
}

impl for_each::Guest for Component {
    fn for_each_string(list: Vec<String>, cb: for_each_cb::OwnCbStringU32ListString) {
        for (i, v) in list.iter().enumerate() {
            (cb.0)(v.as_str(), i as u32, list.as_slice());
        }
    }
    fn for_each_u32(list: Vec<u32>, cb: for_each_cb::OwnCbU32U32ListU32) {
        for (i, v) in list.iter().enumerate() {
            (cb.0)(v.clone(), i as u32, list.as_slice());
        }
    }
}

thread_local!(static MY_BOX_GLOBAL: RefCell<my_box::OwnMyBox> = RefCell::new(my_box::OwnMyBox::new(MyBox::new(0))));
impl my_box::Guest for Component {
    fn set_global(value: my_box::OwnMyBox) {
        MY_BOX_GLOBAL.with(|global| {
            *global.borrow_mut() = value;
        });
    }
    fn global_value() -> u32 {
        MY_BOX_GLOBAL.with(|global| global.borrow().value())
    }
}

pub struct MyBox(u32);
impl my_box::GuestMyBox for MyBox {
    fn new(value: u32) -> Self {
        Self(value)
    }
    fn value(&self) -> u32 {
        self.0
    }
}
