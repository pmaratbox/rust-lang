fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let (num, den) = (6i64, 8i64);
    let g = gcd(num, den);
    println!("{}/{}", num / g, den / g);
}
