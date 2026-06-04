fn factorial_cps(n: u64, k: Box<dyn FnOnce(u64) -> u64>) -> u64 {
    if n == 0 {
        k(1)
    } else {
        factorial_cps(n - 1, Box::new(move |result| k(n * result)))
    }
}

fn main() {
    let answer = factorial_cps(5, Box::new(|x| x));
    println!("{}", answer);
}
