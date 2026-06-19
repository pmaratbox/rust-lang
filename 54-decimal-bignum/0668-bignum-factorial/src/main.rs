use num_bigint::BigInt;
use num_traits::One;

fn main() {
    // Compute 30! exactly with an arbitrary-precision integer.
    let mut f = BigInt::one();
    for i in 1..=30 {
        f *= BigInt::from(i);
    }
    println!("{}", f); // 265252859812191058636308480000000
}
