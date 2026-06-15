use serde::Serialize;

#[derive(Serialize)]
struct Person {
    // Code field uses Rust's camelCase-ish name; serde renames it to the
    // JSON key `full_name` on serialization.
    #[serde(rename = "full_name")]
    name: String,
}

fn main() {
    let p = Person { name: "alice".into() };
    println!("{}", serde_json::to_string(&p).unwrap());
}
