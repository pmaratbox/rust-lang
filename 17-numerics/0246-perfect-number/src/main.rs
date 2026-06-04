fn is_perfect(n: u64) -> bool {
    let sum: u64 = (1..n).filter(|d| n % d == 0).sum();
    sum == n
}

fn label(b: bool) -> &'static str {
    if b {
        "yes"
    } else {
        "no"
    }
}

fn main() {
    println!("{} {}", label(is_perfect(6)), label(is_perfect(8)));
}
