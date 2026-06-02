use std::collections::BTreeMap;

fn main() {
    let word = "banana";
    let mut counts: BTreeMap<char, i32> = BTreeMap::new();
    for ch in word.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }

    let out: Vec<String> = counts
        .iter()
        .map(|(ch, n)| format!("{}:{}", ch, n))
        .collect();
    println!("{}", out.join(" "));
}
