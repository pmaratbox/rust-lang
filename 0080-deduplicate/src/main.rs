use std::collections::HashSet;

fn main() {
    let nums = [1, 2, 2, 3, 1];
    let mut seen = HashSet::new();
    let unique: Vec<i32> = nums.into_iter().filter(|n| seen.insert(*n)).collect();
    let parts: Vec<String> = unique.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
