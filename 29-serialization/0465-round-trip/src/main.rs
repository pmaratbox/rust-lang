use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
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
    let parsed: Person = serde_json::from_str(&json).unwrap();
    println!("{}", parsed.name);
}
