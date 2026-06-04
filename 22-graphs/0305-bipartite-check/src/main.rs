use std::collections::VecDeque;

fn is_bipartite(n: usize, edges: &[(usize, usize)]) -> bool {
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(u, v) in edges {
        adj[u].push(v);
        adj[v].push(u);
    }
    let mut color = vec![-1i8; n];
    for s in 0..n {
        if color[s] != -1 {
            continue;
        }
        color[s] = 0;
        let mut queue = VecDeque::new();
        queue.push_back(s);
        while let Some(u) = queue.pop_front() {
            for &v in &adj[u] {
                if color[v] == -1 {
                    color[v] = 1 - color[u];
                    queue.push_back(v);
                } else if color[v] == color[u] {
                    return false;
                }
            }
        }
    }
    true
}

fn main() {
    let square = [(0, 1), (1, 2), (2, 3), (3, 0)];
    let triangle = [(0, 1), (1, 2), (2, 0)];
    let a = if is_bipartite(4, &square) { "yes" } else { "no" };
    let b = if is_bipartite(3, &triangle) { "yes" } else { "no" };
    println!("{} {}", a, b);
}
