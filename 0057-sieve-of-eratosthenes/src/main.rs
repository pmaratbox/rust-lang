fn main() {
    let n = 10;
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            while j <= n {
                is_prime[j] = false;
                j += i;
            }
        }
        i += 1;
    }

    let primes: Vec<String> = (2..=n)
        .filter(|&i| is_prime[i])
        .map(|i| i.to_string())
        .collect();
    println!("{}", primes.join(" "));
}
