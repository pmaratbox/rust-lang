use std::collections::HashMap;

fn main() {
    let input = "aab";
    let mut order: Vec<char> = Vec::new();
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in input.chars() {
        if !counts.contains_key(&c) {
            order.push(c);
        }
        *counts.entry(c).or_insert(0) += 1;
    }
    let parts: Vec<String> = order
        .iter()
        .map(|c| format!("{}:{}", c, counts[c]))
        .collect();
    println!("{}", parts.join(" "));
}
