use serde::Deserialize;

// Fields declared alphabetically so declaration-order serializers stay alphabetical.
// `age` is missing from the input JSON, so serde fills it via #[serde(default)],
// which uses the type's Default (i32 -> 0).
#[derive(Deserialize)]
struct Person {
    #[serde(default)]
    age: i32,
    name: String,
}

fn main() {
    let json = r#"{"name":"alice"}"#;
    let person: Person = serde_json::from_str(json).unwrap();
    println!("{} {}", person.name, person.age);
}
