fn main() {
    let (rows, cols) = (3usize, 3usize);
    let mut dp = vec![1u64; cols];
    for _ in 1..rows {
        for c in 1..cols {
            dp[c] += dp[c - 1];
        }
    }
    println!("{}", dp[cols - 1]);
}
