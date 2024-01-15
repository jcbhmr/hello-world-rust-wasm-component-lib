#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use wasmtime::component::*;
use wasmtime::{Config, Engine, Store};
pub struct HelloWorldRustWasmComponentLibWorld {
    interface0: exports::jcbhmr::hello_world_rust_wasm_component_lib::cb::Cb,
    interface1: exports::jcbhmr::hello_world_rust_wasm_component_lib::hello_world_rust_wasm_component_lib::HelloWorldRustWasmComponentLib,
}
const _: () = {
    use wasmtime::component::__internal::anyhow;
    impl HelloWorldRustWasmComponentLibWorld {
        pub fn add_to_linker<T, U>(
            linker: &mut wasmtime::component::Linker<T>,
            get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
        ) -> wasmtime::Result<()>
        where
            U: jcbhmr::hello_world_rust_wasm_component_lib::cbh::Host,
        {
            jcbhmr::hello_world_rust_wasm_component_lib::cbh::add_to_linker(
                linker,
                get,
            )?;
            Ok(())
        }
        /// Instantiates the provided `module` using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            component: &wasmtime::component::Component,
            linker: &wasmtime::component::Linker<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = linker.instantiate(&mut store, component)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Instantiates a pre-instantiated module using the specified
        /// parameters, wrapping up the result in a structure that
        /// translates between wasm and the host.
        pub fn instantiate_pre<T>(
            mut store: impl wasmtime::AsContextMut<Data = T>,
            instance_pre: &wasmtime::component::InstancePre<T>,
        ) -> wasmtime::Result<(Self, wasmtime::component::Instance)> {
            let instance = instance_pre.instantiate(&mut store)?;
            Ok((Self::new(store, &instance)?, instance))
        }
        /// Low-level creation wrapper for wrapping up the exports
        /// of the `instance` provided in this structure of wasm
        /// exports.
        ///
        /// This function will extract exports from the `instance`
        /// defined within `store` and wrap them all up in the
        /// returned structure which can be used to interact with
        /// the wasm module.
        pub fn new(
            mut store: impl wasmtime::AsContextMut,
            instance: &wasmtime::component::Instance,
        ) -> wasmtime::Result<Self> {
            let mut store = store.as_context_mut();
            let mut exports = instance.exports(&mut store);
            let mut __exports = exports.root();
            let interface0 = exports::jcbhmr::hello_world_rust_wasm_component_lib::cb::Cb::new(
                &mut __exports
                    .instance("jcbhmr:hello-world-rust-wasm-component-lib/cb")
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!(
                                "exported instance `jcbhmr:hello-world-rust-wasm-component-lib/cb` not present",
                            ),
                        );
                        error
                    }))?,
            )?;
            let interface1 = exports::jcbhmr::hello_world_rust_wasm_component_lib::hello_world_rust_wasm_component_lib::HelloWorldRustWasmComponentLib::new(
                &mut __exports
                    .instance(
                        "jcbhmr:hello-world-rust-wasm-component-lib/hello-world-rust-wasm-component-lib",
                    )
                    .ok_or_else(|| ::anyhow::__private::must_use({
                        let error = ::anyhow::__private::format_err(
                            format_args!(
                                "exported instance `jcbhmr:hello-world-rust-wasm-component-lib/hello-world-rust-wasm-component-lib` not present",
                            ),
                        );
                        error
                    }))?,
            )?;
            Ok(HelloWorldRustWasmComponentLibWorld {
                interface0,
                interface1,
            })
        }
        pub fn jcbhmr_hello_world_rust_wasm_component_lib_cb(
            &self,
        ) -> &exports::jcbhmr::hello_world_rust_wasm_component_lib::cb::Cb {
            &self.interface0
        }
        pub fn jcbhmr_hello_world_rust_wasm_component_lib_hello_world_rust_wasm_component_lib(
            &self,
        ) -> &exports::jcbhmr::hello_world_rust_wasm_component_lib::hello_world_rust_wasm_component_lib::HelloWorldRustWasmComponentLib {
            &self.interface1
        }
    }
};
pub mod jcbhmr {
    pub mod hello_world_rust_wasm_component_lib {
        #[allow(clippy::all)]
        pub mod cbh {
            #[allow(unused_imports)]
            use wasmtime::component::__internal::anyhow;
            pub enum PString {}
            pub trait HostPString {
                fn call(
                    &mut self,
                    self_: wasmtime::component::Resource<PString>,
                    a: String,
                ) -> wasmtime::Result<()>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<PString>,
                ) -> wasmtime::Result<()>;
            }
            pub enum RString {}
            pub trait HostRString {
                fn call(
                    &mut self,
                    self_: wasmtime::component::Resource<RString>,
                ) -> wasmtime::Result<String>;
                fn drop(
                    &mut self,
                    rep: wasmtime::component::Resource<RString>,
                ) -> wasmtime::Result<()>;
            }
            pub trait Host: HostPString + HostRString {}
            pub fn add_to_linker<T, U>(
                linker: &mut wasmtime::component::Linker<T>,
                get: impl Fn(&mut T) -> &mut U + Send + Sync + Copy + 'static,
            ) -> wasmtime::Result<()>
            where
                U: Host,
            {
                let mut inst = linker
                    .instance("jcbhmr:hello-world-rust-wasm-component-lib/cbh")?;
                inst.resource::<
                        PString,
                    >(
                    "p-string",
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostPString::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.resource::<
                        RString,
                    >(
                    "r-string",
                    move |mut store, rep| -> wasmtime::Result<()> {
                        HostRString::drop(
                            get(store.data_mut()),
                            wasmtime::component::Resource::new_own(rep),
                        )
                    },
                )?;
                inst.func_wrap(
                    "[method]p-string.call",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0, arg1): (wasmtime::component::Resource<PString>, String)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostPString::call(host, arg0, arg1);
                        r
                    },
                )?;
                inst.func_wrap(
                    "[method]r-string.call",
                    move |
                        mut caller: wasmtime::StoreContextMut<'_, T>,
                        (arg0,): (wasmtime::component::Resource<RString>,)|
                    {
                        let host = get(caller.data_mut());
                        let r = HostRString::call(host, arg0);
                        Ok((r?,))
                    },
                )?;
                Ok(())
            }
        }
    }
}
pub mod exports {
    pub mod jcbhmr {
        pub mod hello_world_rust_wasm_component_lib {
            #[allow(clippy::all)]
            pub mod cb {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type HPString = super::super::super::super::jcbhmr::hello_world_rust_wasm_component_lib::cbh::PString;
                pub type HRString = super::super::super::super::jcbhmr::hello_world_rust_wasm_component_lib::cbh::RString;
                pub type PString = wasmtime::component::ResourceAny;
                pub struct GuestPString<'a> {
                    funcs: &'a Cb,
                }
                pub type RString = wasmtime::component::ResourceAny;
                pub struct GuestRString<'a> {
                    funcs: &'a Cb,
                }
                pub struct Cb {
                    constructor_p_string_constructor: wasmtime::component::Func,
                    method_p_string_call: wasmtime::component::Func,
                    constructor_r_string_constructor: wasmtime::component::Func,
                    method_r_string_call: wasmtime::component::Func,
                }
                impl Cb {
                    pub fn new(
                        __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                    ) -> wasmtime::Result<Cb> {
                        let constructor_p_string_constructor = *__exports
                            .typed_func::<
                                (wasmtime::component::Resource<HPString>,),
                                (wasmtime::component::ResourceAny,),
                            >("[constructor]p-string")?
                            .func();
                        let method_p_string_call = *__exports
                            .typed_func::<
                                (wasmtime::component::ResourceAny, &str),
                                (),
                            >("[method]p-string.call")?
                            .func();
                        let constructor_r_string_constructor = *__exports
                            .typed_func::<
                                (wasmtime::component::Resource<HRString>,),
                                (wasmtime::component::ResourceAny,),
                            >("[constructor]r-string")?
                            .func();
                        let method_r_string_call = *__exports
                            .typed_func::<
                                (wasmtime::component::ResourceAny,),
                                (String,),
                            >("[method]r-string.call")?
                            .func();
                        Ok(Cb {
                            constructor_p_string_constructor,
                            method_p_string_call,
                            constructor_r_string_constructor,
                            method_r_string_call,
                        })
                    }
                    pub fn p_string(&self) -> GuestPString<'_> {
                        GuestPString { funcs: self }
                    }
                    pub fn r_string(&self) -> GuestRString<'_> {
                        GuestRString { funcs: self }
                    }
                }
                impl GuestPString<'_> {
                    pub fn call_constructor<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::Resource<HPString>,
                    ) -> wasmtime::Result<wasmtime::component::ResourceAny> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::Resource<HPString>,),
                                (wasmtime::component::ResourceAny,),
                            >::new_unchecked(self.funcs.constructor_p_string_constructor)
                        };
                        let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(ret0)
                    }
                    pub fn call_call<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::ResourceAny,
                        arg1: &str,
                    ) -> wasmtime::Result<()> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::ResourceAny, &str),
                                (),
                            >::new_unchecked(self.funcs.method_p_string_call)
                        };
                        let () = callee.call(store.as_context_mut(), (arg0, arg1))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(())
                    }
                }
                impl GuestRString<'_> {
                    pub fn call_constructor<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::Resource<HRString>,
                    ) -> wasmtime::Result<wasmtime::component::ResourceAny> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::Resource<HRString>,),
                                (wasmtime::component::ResourceAny,),
                            >::new_unchecked(self.funcs.constructor_r_string_constructor)
                        };
                        let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(ret0)
                    }
                    pub fn call_call<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::ResourceAny,
                    ) -> wasmtime::Result<String> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::ResourceAny,),
                                (String,),
                            >::new_unchecked(self.funcs.method_r_string_call)
                        };
                        let (ret0,) = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(ret0)
                    }
                }
            }
            #[allow(clippy::all)]
            pub mod hello_world_rust_wasm_component_lib {
                #[allow(unused_imports)]
                use wasmtime::component::__internal::anyhow;
                pub type PString = super::super::super::super::exports::jcbhmr::hello_world_rust_wasm_component_lib::cb::PString;
                pub type RString = super::super::super::super::exports::jcbhmr::hello_world_rust_wasm_component_lib::cb::RString;
                pub struct HelloWorldRustWasmComponentLib {
                    set_cb: wasmtime::component::Func,
                    run_cb_with_result_of: wasmtime::component::Func,
                }
                impl HelloWorldRustWasmComponentLib {
                    pub fn new(
                        __exports: &mut wasmtime::component::ExportInstance<'_, '_>,
                    ) -> wasmtime::Result<HelloWorldRustWasmComponentLib> {
                        let set_cb = *__exports
                            .typed_func::<
                                (wasmtime::component::ResourceAny,),
                                (),
                            >("set-cb")?
                            .func();
                        let run_cb_with_result_of = *__exports
                            .typed_func::<
                                (wasmtime::component::ResourceAny,),
                                (),
                            >("run-cb-with-result-of")?
                            .func();
                        Ok(HelloWorldRustWasmComponentLib {
                            set_cb,
                            run_cb_with_result_of,
                        })
                    }
                    pub fn call_set_cb<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::ResourceAny,
                    ) -> wasmtime::Result<()> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::ResourceAny,),
                                (),
                            >::new_unchecked(self.set_cb)
                        };
                        let () = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(())
                    }
                    pub fn call_run_cb_with_result_of<S: wasmtime::AsContextMut>(
                        &self,
                        mut store: S,
                        arg0: wasmtime::component::ResourceAny,
                    ) -> wasmtime::Result<()> {
                        let callee = unsafe {
                            wasmtime::component::TypedFunc::<
                                (wasmtime::component::ResourceAny,),
                                (),
                            >::new_unchecked(self.run_cb_with_result_of)
                        };
                        let () = callee.call(store.as_context_mut(), (arg0,))?;
                        callee.post_return(store.as_context_mut())?;
                        Ok(())
                    }
                }
            }
        }
    }
}
const _: &str = "package jcbhmr:hello-world-rust-wasm-component-lib;\n\ninterface cbh {\n  resource p-string {\n    call: func(a: string);\n  }\n  resource r-string {\n    call: func() -> string;\n  }\n}\ninterface cb {\n  use cbh.{p-string as h-p-string, r-string as h-r-string};\n  resource p-string {\n    constructor(cb: h-p-string);\n    call: func(a: string);\n  }\n  resource r-string {\n    constructor(cb: h-r-string);\n    call: func() -> string;\n  }\n}\n\ninterface hello-world-rust-wasm-component-lib {\n  use cb.{p-string, r-string};\n  set-cb: func(cb: p-string);\n  run-cb-with-result-of: func(cb: borrow<r-string>);\n}\n\nworld hello-world-rust-wasm-component-lib-world {\n  import cbh;\n  export cb;\n\n  export hello-world-rust-wasm-component-lib;\n}\n";
struct MyState;
impl Host for MyState {}
extern crate test;
#[cfg(test)]
#[rustc_test_marker = "it_works"]
pub const it_works: test::TestDescAndFn = test::TestDescAndFn {
    desc: test::TestDesc {
        name: test::StaticTestName("it_works"),
        ignore: false,
        ignore_message: ::core::option::Option::None,
        source_file: "tests/hello_world_test.rs",
        start_line: 12usize,
        start_col: 4usize,
        end_line: 12usize,
        end_col: 12usize,
        compile_fail: false,
        no_run: false,
        should_panic: test::ShouldPanic::No,
        test_type: test::TestType::IntegrationTest,
    },
    testfn: test::StaticTestFn(|| test::assert_test_result(it_works())),
};
fn it_works() {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config).unwrap();
    let component = Component::from_file(
            &engine,
            "./target/wasm32-unknown-unknown/hello_world_rust_wasm_component.wasm",
        )
        .unwrap();
}
#[rustc_main]
#[coverage(off)]
pub fn main() -> () {
    extern crate test;
    test::test_main_static(&[&it_works])
}
