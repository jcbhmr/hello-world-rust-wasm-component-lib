use std::f64::consts::PI;

// Use a procedural macro to generate bindings for the world we specified in
// `host.wit`
wit_bindgen::generate!({
    // the name of the world in the `*.wit` input file
    world: "hello-world-rust-wasm-component",

    // For all exported worlds, interfaces, and resources, this specifies what
    // type they're corresponding to in this module. In this case the `MyHost`
    // struct defined below is going to define the exports of the `world`,
    // namely the `run` function.
    exports: {
        world: HelloWorldRustWasmComponent,
        "example:hello-world-rust-wasm-component/hello-world-rust-wasm-component-interface": HelloWorldRustWasmComponent,
    },
});

// Define a custom type and implement the generated `Guest` trait for it which
// represents implementing all the necessary exported interfaces for this
// component.
struct HelloWorldRustWasmComponent;

impl Guest for HelloWorldRustWasmComponent {
    fn run() {
        print("Hello, world!");
    }

    fn greet(name: String) -> String {
        return format!("Hello {}!", name);
    }

    fn greet_many(names: Vec<String>) -> String {
        // Check if the vector is empty
        if names.is_empty() {
            return String::from("No names provided");
        }

        // Iterate through names and build greetings
        let mut greetings = String::new();
        for name in names {
            greetings.push_str(&format!("Hello, {}!\n", name));
        }

        greetings
    }

    fn get_report() -> Report {
        return Report {
            bouncy_castles: 100,
            fun_percent: 0.90,
            cat_count: 8,
            unicorn_names: vec!["Fluffy".into(), "Marshmallow".into(), "Sparkles".into()],
        };
    }

    fn compute_area(circle:Circle,) -> f64 {
        return PI * circle.radius * circle.radius;
    }
}
