fn longest_common_substring(a: &[u8], b: &[u8]) -> String {
    let mut dp = vec![vec![0usize; b.len() + 1]; a.len() + 1];
    let mut best = 0;
    let mut end = 0;
    for i in 1..=a.len() {
        for j in 1..=b.len() {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                if dp[i][j] > best {
                    best = dp[i][j];
                    end = i;
                }
            }
        }
    }
    String::from_utf8(a[end - best..end].to_vec()).unwrap()
}

fn main() {
    println!("{}", longest_common_substring(b"abcde", b"xbcdy"));
}
