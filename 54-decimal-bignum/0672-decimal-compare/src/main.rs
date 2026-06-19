use rust_decimal::Decimal;
use std::str::FromStr;

fn main() {
    let a = Decimal::from_str("0.1").unwrap();
    let b = Decimal::from_str("0.2").unwrap();
    let expected = Decimal::from_str("0.3").unwrap();
    // Exact base-10 decimals: 0.1 + 0.2 is exactly 0.3 (unlike binary floats).
    println!("{}", a + b == expected);
}
