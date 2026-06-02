use std::collections::HashMap;

fn main() {
    let nums = HashMap::from([("one", 1), ("two", 2), ("three", 3)]);

    println!("two: {}", nums["two"]);
    println!("size: {}", nums.len());
}
