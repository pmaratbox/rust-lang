fn expand(s: &[u8], mut lo: i32, mut hi: i32) -> (usize, usize) {
    while lo >= 0 && (hi as usize) < s.len() && s[lo as usize] == s[hi as usize] {
        lo -= 1;
        hi += 1;
    }
    ((lo + 1) as usize, hi as usize)
}

fn longest_palindrome(s: &[u8]) -> String {
    let mut best = (0usize, 0usize);
    for i in 0..s.len() {
        for (lo, hi) in [expand(s, i as i32, i as i32), expand(s, i as i32, i as i32 + 1)] {
            if hi - lo > best.1 - best.0 {
                best = (lo, hi);
            }
        }
    }
    String::from_utf8(s[best.0..best.1].to_vec()).unwrap()
}

fn main() {
    println!("{}", longest_palindrome(b"babad"));
}
