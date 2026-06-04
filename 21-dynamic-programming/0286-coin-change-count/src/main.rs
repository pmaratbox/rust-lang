fn main() {
    let coins = [1, 2, 5];
    let target = 5usize;
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1;
    for &c in &coins {
        for amount in c..=target {
            dp[amount] += dp[amount - c];
        }
    }
    println!("{}", dp[target]);
}
