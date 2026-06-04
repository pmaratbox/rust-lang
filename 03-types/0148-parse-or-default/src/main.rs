fn main() {
    let a: i32 = "42".parse().unwrap_or(0);
    let b: i32 = "x".parse().unwrap_or(0);
    println!("{} {}", a, b);
}
