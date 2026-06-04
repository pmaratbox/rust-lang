fn main() {
    let edges: [(usize, usize, i64); 3] = [(0, 1, 1), (1, 2, -2), (0, 2, 4)];
    let n = 3;
    let inf = i64::MAX;
    let mut dist = vec![inf; n];
    dist[0] = 0;

    for _ in 0..n - 1 {
        for &(u, v, w) in &edges {
            if dist[u] != inf && dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
            }
        }
    }

    let out: Vec<String> = dist.iter().map(|d| d.to_string()).collect();
    println!("{}", out.join(" "));
}
