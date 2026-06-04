use std::collections::BTreeMap;

fn main() {
    let left = [("a", 1), ("b", 2)];
    let right = [("b", 3), ("c", 4)];

    let mut merged: BTreeMap<&str, i32> = BTreeMap::new();
    for (k, v) in left.iter().chain(right.iter()) {
        merged.insert(k, *v);
    }

    let out: Vec<String> = merged.iter().map(|(k, v)| format!("{}:{}", k, v)).collect();
    println!("{}", out.join(" "));
}
