fn main() {
    let items = [(2usize, 3u64), (3, 4), (4, 5)];
    let capacity = 5usize;
    let mut dp = vec![0u64; capacity + 1];
    for &(w, v) in &items {
        for cap in (w..=capacity).rev() {
            dp[cap] = dp[cap].max(dp[cap - w] + v);
        }
    }
    println!("{}", dp[capacity]);
}
