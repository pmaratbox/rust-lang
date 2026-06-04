fn main() {
    let coins = [1, 2, 5];
    let target = 11usize;
    let inf = usize::MAX;
    let mut dp = vec![inf; target + 1];
    dp[0] = 0;
    for amount in 1..=target {
        for &c in &coins {
            if c <= amount && dp[amount - c] != inf {
                dp[amount] = dp[amount].min(dp[amount - c] + 1);
            }
        }
    }
    println!("{}", dp[target]);
}
