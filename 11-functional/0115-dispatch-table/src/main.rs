use std::collections::HashMap;

fn main() {
    let mut table: HashMap<&str, fn(i32, i32) -> i32> = HashMap::new();
    table.insert("add", |a, b| a + b);
    table.insert("mul", |a, b| a * b);

    println!("{} {}", table["add"](3, 4), table["mul"](3, 4));
}
