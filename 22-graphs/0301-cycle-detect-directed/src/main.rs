fn dfs(u: usize, adj: &[Vec<usize>], color: &mut [u8]) -> bool {
    color[u] = 1; // gray
    for &v in &adj[u] {
        if color[v] == 1 {
            return true;
        }
        if color[v] == 0 && dfs(v, adj, color) {
            return true;
        }
    }
    color[u] = 2; // black
    false
}

fn main() {
    let edges: [(usize, usize); 3] = [(0, 1), (1, 2), (2, 0)];
    let n = 3;
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for &(u, v) in &edges {
        adj[u].push(v);
    }

    let mut color = vec![0u8; n];
    let mut has_cycle = false;
    for s in 0..n {
        if color[s] == 0 && dfs(s, &adj, &mut color) {
            has_cycle = true;
            break;
        }
    }

    println!("{}", if has_cycle { "cycle" } else { "acyclic" });
}
