use num_bigint::BigInt;
use num_traits::Pow;

fn main() {
    // 2^100 is far beyond u64/i128, so compute it with an arbitrary-precision integer.
    let result = BigInt::from(2).pow(100u32);
    println!("{}", result);
}
