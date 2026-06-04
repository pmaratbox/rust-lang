// Operation succeeds only on the 3rd attempt.
fn attempt(n: u32) -> bool {
    n >= 3
}

fn main() {
    let max_attempts = 5;
    for n in 1..=max_attempts {
        if attempt(n) {
            println!("ok after {}", n);
            break;
        }
    }
}
