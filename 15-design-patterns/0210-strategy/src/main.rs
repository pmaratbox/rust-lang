fn add(a: i32, b: i32) -> i32 {
    a + b
}
fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn apply(strategy: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    strategy(a, b)
}

fn main() {
    let a = apply(add, 3, 4);
    let b = apply(mul, 3, 4);
    println!("{} {}", a, b);
}
