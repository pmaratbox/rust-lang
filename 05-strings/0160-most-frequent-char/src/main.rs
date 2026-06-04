use std::collections::HashMap;

fn main() {
    let s = "hello";
    let mut counts: HashMap<char, usize> = HashMap::new();
    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }
    let best = s
        .chars()
        .max_by_key(|c| counts[c])
        .unwrap();
    println!("{}", best);
}
