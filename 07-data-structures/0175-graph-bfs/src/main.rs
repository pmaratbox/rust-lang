use std::collections::VecDeque;

fn main() {
    let adj: Vec<Vec<usize>> = vec![
        vec![1, 2], // 0
        vec![0, 3], // 1
        vec![0, 3], // 2
        vec![1, 2], // 3
    ];

    let mut visited = vec![false; adj.len()];
    let mut queue = VecDeque::new();
    let mut order = Vec::new();

    visited[0] = true;
    queue.push_back(0);

    while let Some(node) = queue.pop_front() {
        order.push(node.to_string());
        for &next in &adj[node] {
            if !visited[next] {
                visited[next] = true;
                queue.push_back(next);
            }
        }
    }

    println!("{}", order.join(" "));
}
