use std::collections::HashSet;

fn main() {
    let words = ["eat", "tea", "tan", "ate", "nat"];
    let mut groups: HashSet<String> = HashSet::new();
    for w in words {
        let mut chars: Vec<char> = w.chars().collect();
        chars.sort_unstable();
        groups.insert(chars.into_iter().collect());
    }
    println!("{}", groups.len());
}
