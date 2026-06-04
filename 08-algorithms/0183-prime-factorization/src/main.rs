fn main() {
    let mut n = 60;
    let mut factors = Vec::new();
    while n % 2 == 0 {
        factors.push(2);
        n /= 2;
    }
    let mut i = 3;
    while i * i <= n {
        while n % i == 0 {
            factors.push(i);
            n /= i;
        }
        i += 2;
    }
    if n > 1 {
        factors.push(n);
    }
    let parts: Vec<String> = factors.iter().map(|x| x.to_string()).collect();
    println!("{}", parts.join(" "));
}
