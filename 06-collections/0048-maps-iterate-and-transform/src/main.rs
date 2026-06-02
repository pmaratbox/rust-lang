use std::collections::HashMap;

fn main() {
    let scores = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    let total: i32 = scores.values().sum();
    println!("sum: {total}");
}
