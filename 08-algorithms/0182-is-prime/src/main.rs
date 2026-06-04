fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    let mut i = 2;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    let label = |n| if is_prime(n) { "yes" } else { "no" };
    println!("{} {}", label(7), label(9));
}
