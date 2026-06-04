fn main() {
    let nums = [3, 34, 4, 12, 5, 2];
    let target = 9usize;
    let mut dp = vec![false; target + 1];
    dp[0] = true;
    for &x in &nums {
        for s in (x..=target).rev() {
            if dp[s - x] {
                dp[s] = true;
            }
        }
    }
    println!("{}", if dp[target] { "yes" } else { "no" });
}
