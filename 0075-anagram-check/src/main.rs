fn sorted(s: &str) -> Vec<char> {
    let mut chars: Vec<char> = s.chars().collect();
    chars.sort();
    chars
}

fn main() {
    let pairs = [("listen", "silent"), ("hello", "world")];
    for (a, b) in pairs {
        let ans = if sorted(a) == sorted(b) { "yes" } else { "no" };
        println!("{}/{}: {}", a, b, ans);
    }
}
