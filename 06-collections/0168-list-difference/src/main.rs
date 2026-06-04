use std::collections::HashSet;

fn main() {
    let a = [1, 2, 3, 4];
    let b = [2, 4];
    let remove: HashSet<i32> = b.iter().copied().collect();

    let diff: Vec<String> = a
        .iter()
        .filter(|n| !remove.contains(n))
        .map(|n| n.to_string())
        .collect();
    println!("{}", diff.join(" "));
}
