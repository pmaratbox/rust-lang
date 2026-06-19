use num_bigint::BigInt;
use std::str::FromStr;

fn main() {
    // Both operands exceed u64::MAX, so we parse them into BigInt
    // (an arbitrary-precision signed integer) and add exactly.
    let a = BigInt::from_str("12345678901234567890").unwrap();
    let b = BigInt::from_str("98765432109876543210").unwrap();
    println!("{}", &a + &b);
}
