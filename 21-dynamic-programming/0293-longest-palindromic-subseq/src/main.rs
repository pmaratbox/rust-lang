fn main() {
    let s: Vec<u8> = "bbbab".bytes().collect();
    let n = s.len();
    let mut dp = vec![vec![0usize; n]; n];
    for i in (0..n).rev() {
        dp[i][i] = 1;
        for j in i + 1..n {
            dp[i][j] = if s[i] == s[j] {
                dp[i + 1][j - 1] + 2
            } else {
                dp[i + 1][j].max(dp[i][j - 1])
            };
        }
    }
    println!("{}", dp[0][n - 1]);
}
