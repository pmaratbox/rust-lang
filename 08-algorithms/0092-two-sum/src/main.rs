use std::collections::HashMap;

fn main() {
    let nums = [2, 7, 11, 15];
    let target = 9;
    let mut seen = HashMap::new();
    for (i, &n) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - n)) {
            println!("{} {}", j, i);
            break;
        }
        seen.insert(n, i);
    }
}
