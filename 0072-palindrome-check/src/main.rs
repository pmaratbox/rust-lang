fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let reversed: Vec<char> = s.chars().rev().collect();
    chars == reversed
}

fn main() {
    for word in ["level", "hello"] {
        let ans = if is_palindrome(word) { "yes" } else { "no" };
        println!("{}: {}", word, ans);
    }
}
