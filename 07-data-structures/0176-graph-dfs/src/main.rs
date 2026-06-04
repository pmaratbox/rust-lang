fn dfs(node: usize, adj: &[Vec<usize>], visited: &mut [bool], order: &mut Vec<String>) {
    visited[node] = true;
    order.push(node.to_string());
    for &next in &adj[node] {
        if !visited[next] {
            dfs(next, adj, visited, order);
        }
    }
}

fn main() {
    let adj: Vec<Vec<usize>> = vec![
        vec![1, 2], // 0
        vec![0, 3], // 1
        vec![0, 3], // 2
        vec![1, 2], // 3
    ];

    let mut visited = vec![false; adj.len()];
    let mut order = Vec::new();
    dfs(0, &adj, &mut visited, &mut order);

    println!("{}", order.join(" "));
}
