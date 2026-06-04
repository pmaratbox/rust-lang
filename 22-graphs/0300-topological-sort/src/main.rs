use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let edges: [(usize, usize); 4] = [(0, 1), (0, 2), (1, 3), (2, 3)];
    let n = 4;
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut indeg = vec![0u32; n];
    for &(u, v) in &edges {
        adj[u].push(v);
        indeg[v] += 1;
    }

    let mut heap = BinaryHeap::new();
    for v in 0..n {
        if indeg[v] == 0 {
            heap.push(Reverse(v));
        }
    }

    let mut order = Vec::new();
    while let Some(Reverse(u)) = heap.pop() {
        order.push(u);
        for &v in &adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                heap.push(Reverse(v));
            }
        }
    }

    let out: Vec<String> = order.iter().map(|v| v.to_string()).collect();
    println!("{}", out.join(" "));
}
