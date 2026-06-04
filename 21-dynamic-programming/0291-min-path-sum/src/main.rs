fn main() {
    let grid = [[1, 3, 1], [1, 5, 1], [4, 2, 1]];
    let rows = grid.len();
    let cols = grid[0].len();
    let mut dp = vec![vec![0u64; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            let cell = grid[r][c] as u64;
            dp[r][c] = match (r, c) {
                (0, 0) => cell,
                (0, _) => dp[r][c - 1] + cell,
                (_, 0) => dp[r - 1][c] + cell,
                _ => dp[r - 1][c].min(dp[r][c - 1]) + cell,
            };
        }
    }
    println!("{}", dp[rows - 1][cols - 1]);
}
