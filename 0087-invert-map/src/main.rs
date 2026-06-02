use std::collections::{HashMap, BTreeMap};

fn main() {
    let m = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    let inverted: BTreeMap<i32, &str> = m.into_iter().map(|(k, v)| (v, k)).collect();
    let parts: Vec<String> = inverted.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
    println!("{}", parts.join(" "));
}
