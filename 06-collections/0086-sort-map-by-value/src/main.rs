use std::collections::HashMap;

fn main() {
    let scores = HashMap::from([("a", 3), ("b", 1), ("c", 2)]);
    let mut entries: Vec<(&str, i32)> = scores.into_iter().collect();
    entries.sort_by_key(|&(_, v)| v);
    let parts: Vec<String> = entries.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
    println!("{}", parts.join(" "));
}
