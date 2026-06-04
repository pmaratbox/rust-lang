fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let (n1, d1) = (1i64, 2i64);
    let (n2, d2) = (1i64, 3i64);
    let mut num = n1 * d2 + n2 * d1;
    let mut den = d1 * d2;
    let g = gcd(num, den);
    num /= g;
    den /= g;
    println!("{}/{}", num, den);
}
