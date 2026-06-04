fn main() {
    let (a, b) = (1i64, 2i64);
    let (c, d) = (3i64, 4i64);
    let real = a * c - b * d;
    let imag = a * d + b * c;
    println!("{} {}", real, imag);
}
