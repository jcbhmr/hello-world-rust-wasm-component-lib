wai_bindgen_rust::export!("exports.wai");
pub struct Exports;
impl exports::Exports for Exports {
    fn add(a: u32, b: u32) -> u32 {
        a + b
    }

    fn greet(name: String) -> String {
        format!("Hello {name}!")
    }

    fn greet_many(people: Vec<String>) -> String {
        match people.as_slice() {
            [] => "Hello!".to_string(),
            [person] => Exports::greet(person.into()),
            [person_1, person_2] => format!("Hello {person_1} and {person_2}!"),
            [people @ .., last] => {
                let people = people.join(", ");
                format!("Hello {people}, and {last}!")
            }
        }
    }

    fn distance_between(p1: exports::Point, p2: exports::Point) -> f32 {
        let exports::Point { x: x1, y: y1 } = p1;
        let exports::Point { x: x2, y: y2 } = p2;

        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
}
