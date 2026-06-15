use serde::Serialize;

// Fields declared alphabetically so serde's declaration-order output
// yields keys in alphabetical order: age before name.
#[derive(Serialize)]
struct Person {
    age: i32,
    name: String,
}

fn main() {
    let person = Person {
        age: 30,
        name: "alice".into(),
    };
    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);
}
