use serde::Serialize;

// Fields declared in alphabetical order so serde_json emits keys
// alphabetically in compact form.
#[derive(Serialize)]
struct Person {
    age: i32,
    name: String,
}

fn main() {
    let people = vec![
        Person { age: 30, name: "alice".into() },
        Person { age: 25, name: "bob".into() },
    ];
    println!("{}", serde_json::to_string(&people).unwrap());
}
