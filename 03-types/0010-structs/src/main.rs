struct Person {
    name: String,
    age: u32,
}

fn main() {
    let p = Person {
        name: String::from("Ada"),
        age: 36,
    };

    println!("name: {}", p.name);
    println!("age: {}", p.age);
}
