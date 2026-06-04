use std::collections::BinaryHeap;

fn main() {
    let mut heap: BinaryHeap<i64> = BinaryHeap::from(vec![3, 1, 4, 1, 5]);
    let top3: Vec<String> = (0..3)
        .filter_map(|_| heap.pop())
        .map(|v| v.to_string())
        .collect();
    println!("{}", top3.join(" "));
}
