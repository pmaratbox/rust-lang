fn main() {
    let dims = [10, 20, 30, 40];
    let n = dims.len() - 1; // number of matrices
    let mut dp = vec![vec![0u64; n]; n];
    for len in 2..=n {
        for i in 0..=n - len {
            let j = i + len - 1;
            dp[i][j] = u64::MAX;
            for k in i..j {
                let cost =
                    dp[i][k] + dp[k + 1][j] + dims[i] * dims[k + 1] * dims[j + 1];
                dp[i][j] = dp[i][j].min(cost);
            }
        }
    }
    println!("{}", dp[0][n - 1]);
}
