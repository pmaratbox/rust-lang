use serde::Deserialize;

// Fields declared in alphabetical order to keep canonical JSON key ordering.
#[derive(Deserialize)]
struct Person {
    age: i32,
    name: String,
}

fn main() {
    let json = r#"{"age":30,"name":"alice"}"#;
    let person: Person = serde_json::from_str(json).unwrap();
    println!("{} {}", person.name, person.age);
}
