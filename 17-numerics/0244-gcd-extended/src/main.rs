fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 {
        (a, 1, 0)
    } else {
        let (g, x, y) = ext_gcd(b, a % b);
        (g, y, x - (a / b) * y)
    }
}

fn main() {
    let (g, x, y) = ext_gcd(30, 12);
    println!("{} {} {}", g, x, y);
}
