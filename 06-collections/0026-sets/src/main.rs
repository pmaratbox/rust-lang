use std::collections::HashSet;

fn main() {
    let nums: HashSet<i32> = HashSet::from([1, 2, 2, 3]);
    println!("size: {}", nums.len());
    println!("has 2: {}", if nums.contains(&2) { "yes" } else { "no" });
    println!("has 5: {}", if nums.contains(&5) { "yes" } else { "no" });
}
