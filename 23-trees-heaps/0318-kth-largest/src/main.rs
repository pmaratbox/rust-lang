use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn kth_largest(data: &[i64], k: usize) -> i64 {
    // Min-heap (via Reverse) holding the k largest values.
    let mut heap: BinaryHeap<Reverse<i64>> = BinaryHeap::new();
    for &v in data {
        heap.push(Reverse(v));
        if heap.len() > k {
            heap.pop();
        }
    }
    heap.peek().map(|Reverse(v)| *v).unwrap()
}

fn main() {
    let data = [3, 2, 1, 5, 6, 4];
    println!("{}", kth_largest(&data, 2));
}
