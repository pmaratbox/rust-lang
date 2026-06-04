use std::collections::HashSet;

fn main() {
    let mut n: u32 = 19;
    let mut seen = HashSet::new();
    while n != 1 && seen.insert(n) {
        let mut sum = 0;
        while n > 0 {
            let d = n % 10;
            sum += d * d;
            n /= 10;
        }
        n = sum;
    }
    println!("{}", if n == 1 { "yes" } else { "no" });
}
