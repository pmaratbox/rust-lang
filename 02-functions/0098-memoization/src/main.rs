use std::collections::HashMap;

fn fib(n: u64, cache: &mut HashMap<u64, u64>) -> u64 {
    if n < 2 {
        return n;
    }
    if let Some(&v) = cache.get(&n) {
        return v;
    }
    let result = fib(n - 1, cache) + fib(n - 2, cache);
    cache.insert(n, result);
    result
}

fn main() {
    let mut cache = HashMap::new();
    println!("{}", fib(10, &mut cache));
}
