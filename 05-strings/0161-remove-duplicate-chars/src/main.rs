use std::collections::HashSet;

fn main() {
    let s = "aabbcc";
    let mut seen = HashSet::new();
    let result: String = s.chars().filter(|c| seen.insert(*c)).collect();
    println!("{}", result);
}
