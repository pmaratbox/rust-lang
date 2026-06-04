fn main() {
    let nums = [1, 5, 11, 5];
    let total: usize = nums.iter().sum();
    let answer = if total % 2 != 0 {
        false
    } else {
        let target = total / 2;
        let mut dp = vec![false; target + 1];
        dp[0] = true;
        for &x in &nums {
            for s in (x..=target).rev() {
                if dp[s - x] {
                    dp[s] = true;
                }
            }
        }
        dp[target]
    };
    println!("{}", if answer { "yes" } else { "no" });
}
