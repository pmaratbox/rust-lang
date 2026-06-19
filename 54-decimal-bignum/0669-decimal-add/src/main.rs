use rust_decimal::Decimal;
use std::str::FromStr;

fn main() {
    // Binary floats can't represent 0.1 or 0.2 exactly, so `0.1_f64 + 0.2_f64`
    // yields 0.30000000000000004. rust_decimal::Decimal is a base-10 exact type,
    // so parsing the literals and adding gives the precise result 0.3.
    let a = Decimal::from_str("0.1").unwrap();
    let b = Decimal::from_str("0.2").unwrap();
    println!("{}", a + b);
}
