fn main() {
    // Undirected graph as an adjacency list indexed by node id.
    let adj: Vec<Vec<usize>> = vec![
        vec![1, 2], // 0
        vec![0, 3], // 1
        vec![0, 3], // 2
        vec![1, 2], // 3
    ];

    let neighbors: Vec<String> = adj[0].iter().map(|n| n.to_string()).collect();
    println!("{}", neighbors.join(" "));
}
