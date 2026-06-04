struct Address {
    city: String,
}

struct Person {
    name: String,
    address: Address,
}

fn main() {
    let person = Person {
        name: String::from("Ada"),
        address: Address {
            city: String::from("London"),
        },
    };
    let _ = &person.name;
    println!("{}", person.address.city);
}
