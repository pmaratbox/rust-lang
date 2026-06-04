use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn main() {
    let edges: [(usize, usize, u32); 5] =
        [(0, 1, 4), (0, 2, 1), (2, 1, 2), (1, 3, 1), (2, 3, 5)];
    let n = 4;
    let mut adj: Vec<Vec<(usize, u32)>> = vec![Vec::new(); n];
    for &(u, v, w) in &edges {
        adj[u].push((v, w));
    }

    let mut dist = vec![u32::MAX; n];
    dist[0] = 0;
    let mut heap = BinaryHeap::new();
    heap.push(Reverse((0u32, 0usize)));

    while let Some(Reverse((d, u))) = heap.pop() {
        if d > dist[u] {
            continue;
        }
        for &(v, w) in &adj[u] {
            let nd = d + w;
            if nd < dist[v] {
                dist[v] = nd;
                heap.push(Reverse((nd, v)));
            }
        }
    }

    let out: Vec<String> = dist.iter().map(|d| d.to_string()).collect();
    println!("{}", out.join(" "));
}
