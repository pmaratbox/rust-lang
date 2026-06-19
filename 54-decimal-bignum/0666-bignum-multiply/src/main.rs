use num_bigint::BigInt;

fn main() {
    let a = BigInt::from(123456789);
    let b = BigInt::from(987654321);
    println!("{}", a * b);
}
