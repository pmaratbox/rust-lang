use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    let mut heap = BinaryHeap::new();
    for n in [3, 1, 2] {
        heap.push(Reverse(n));
    }

    let mut out = Vec::new();
    while let Some(Reverse(n)) = heap.pop() {
        out.push(n.to_string());
    }
    println!("{}", out.join(" "));
}
