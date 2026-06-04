fn main() {
    let n = 3;
    let inf = i64::MAX / 4;
    let mut dist = vec![vec![inf; n]; n];
    for i in 0..n {
        dist[i][i] = 0;
    }
    let edges: [(usize, usize, i64); 3] = [(0, 1, 3), (1, 2, 1), (0, 2, 5)];
    for &(u, v, w) in &edges {
        dist[u][v] = w;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                let through = dist[i][k] + dist[k][j];
                if through < dist[i][j] {
                    dist[i][j] = through;
                }
            }
        }
    }

    println!("{}", dist[0][2]);
}
