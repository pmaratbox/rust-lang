use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let edges: [(usize, usize, u32); 3] = [(0, 1, 1), (1, 2, 2), (2, 3, 3)];
    let n = 4;
    let mut adj: Vec<Vec<(usize, u32)>> = vec![Vec::new(); n];
    for &(u, v, w) in &edges {
        adj[u].push((v, w));
        adj[v].push((u, w));
    }

    let mut in_tree = vec![false; n];
    let mut total = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u32, 0usize)));

    while let Some(Reverse((w, u))) = heap.pop() {
        if in_tree[u] {
            continue;
        }
        in_tree[u] = true;
        total += w;
        for &(v, ew) in &adj[u] {
            if !in_tree[v] {
                heap.push(Reverse((ew, v)));
            }
        }
    }

    println!("{}", total);
}
