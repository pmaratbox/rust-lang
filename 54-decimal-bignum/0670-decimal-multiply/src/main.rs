use rust_decimal::Decimal;
use std::str::FromStr;

fn main() {
    let a = Decimal::from_str("1.1").unwrap();
    let b = Decimal::from_str("1.1").unwrap();
    // Exact decimal multiplication: no binary floating-point rounding.
    println!("{}", a * b);
}
