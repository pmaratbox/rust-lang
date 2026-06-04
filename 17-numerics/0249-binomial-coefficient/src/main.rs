fn binomial(n: u64, k: u64) -> u64 {
    let k = k.min(n - k);
    let mut result = 1u64;
    for i in 0..k {
        result = result * (n - i) / (i + 1);
    }
    result
}

fn main() {
    println!("{}", binomial(5, 2));
}
