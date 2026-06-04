use std::collections::HashMap;

fn main() {
    let xs = [1, 1, 2, 3, 3, 3];

    let mut counts: HashMap<i32, usize> = HashMap::new();
    let mut order: Vec<i32> = Vec::new();
    for &x in &xs {
        if counts.insert(x, *counts.get(&x).unwrap_or(&0) + 1).is_none() {
            order.push(x);
        }
    }

    // Stable sort by descending count; ties keep first-seen order.
    order.sort_by(|a, b| counts[b].cmp(&counts[a]));

    let parts: Vec<String> = order
        .iter()
        .flat_map(|&v| std::iter::repeat(v.to_string()).take(counts[&v]))
        .collect();
    println!("{}", parts.join(" "));
}
