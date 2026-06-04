fn main() {
    // 2x^2 + 3x + 1, coefficients from highest to lowest degree
    let coeffs = [2i64, 3, 1];
    let x = 2i64;
    let value = coeffs.iter().fold(0i64, |acc, &c| acc * x + c);
    println!("{}", value);
}
