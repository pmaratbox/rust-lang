use std::collections::HashMap;

fn main() {
    let items = ["a", "b", "a", "c", "b", "a"];

    let mut counts: HashMap<&str, usize> = HashMap::new();
    for &item in &items {
        *counts.entry(item).or_insert(0) += 1;
    }

    let mut pairs: Vec<(&str, usize)> = counts.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));

    let top: Vec<&str> = pairs.iter().take(2).map(|(k, _)| *k).collect();
    println!("{}", top.join(" "));
}
