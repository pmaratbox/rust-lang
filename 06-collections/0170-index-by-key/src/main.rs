use std::collections::HashMap;

fn main() {
    let people = [(1, "alice"), (2, "bob")];
    let by_id: HashMap<i32, &str> = people.iter().copied().collect();
    println!("id 2: {}", by_id[&2]);
}
