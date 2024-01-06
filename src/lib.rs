cargo_component_bindings::generate!();
use bindings::exports::*;
use bindings::*;

struct Component;
impl Guest for Component {
    fn say_hi() {
        print("Hello from Rust!");
    }

    fn greet_many(names: Vec<String>) -> String {
        return names
            .into_iter()
            .map(|name| Self::greet(name))
            .collect::<Vec<String>>()
            .join("\n");
    }

    fn greet(name: String) -> String {
        return format!("Hello {}!", name);
    }
}

pub struct IntBox {
    value: i32,
}
impl my_interface::GuestIntBox for IntBox {
    fn new(a: i32) -> Self {
        Self { value: a }
    }
    fn get_value(&self) -> i32 {
        self.value
    }
}
