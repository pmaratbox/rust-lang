use serde::Serialize;

#[derive(Serialize)]
struct Address {
    city: String,
    zip: i32,
}

#[derive(Serialize)]
struct Person {
    address: Address,
    name: String,
}

fn main() {
    let person = Person {
        address: Address {
            city: "oslo".into(),
            zip: 1000,
        },
        name: "alice".into(),
    };
    println!("{}", serde_json::to_string(&person).unwrap());
}
