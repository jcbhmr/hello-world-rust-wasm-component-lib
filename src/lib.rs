cargo_component_bindings::generate!();
use bindings::exports::jcbhmr::hello_world_rust_wasm_component_lib::*;
use bindings::jcbhmr::hello_world_rust_wasm_component_lib::*;
use derive_more::{From, Into};
use std::cell::RefCell;

pub struct Component;

#[derive(From, Into)]
pub struct PString(Box<dyn Fn(&str)>);
impl cb::GuestPString for PString {
    fn new(cb: cbh::PString) -> Self {
        Self(Box::new(move |a| cb.call(a)))
    }
    fn call(&self, s: String) {
        (self.0)(s.as_str());
    }
}

#[derive(From, Into)]
pub struct RString(Box<dyn Fn() -> String>);
impl cb::GuestRString for RString {
    fn new(cb: cbh::RString) -> Self {
        Self(Box::new(move || cb.call()))
    }
    fn call(&self) -> String {
        (self.0)()
    }
}

thread_local! {
    static CB: RefCell<Option<cb::OwnPString>> = RefCell::new(None);
}

impl hello_world_rust_wasm_component_lib::Guest for Component {
    fn set_cb(cb: cb::OwnPString) {
        CB.with(|cb2| {
            *cb2.borrow_mut() = Some(cb);
        });
    }
    fn run_cb_with_result_of(cb: &cb::RString) {
        let res = (cb.0)();
        CB.with(|cb2| {
            if let Some(cb) = &*cb2.borrow() {
                (cb.0)(res.as_str());
            }
        });
    }
}
