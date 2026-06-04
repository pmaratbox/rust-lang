fn main() {
    let mut a: u32 = 3;
    let mut b: u32 = 5;
    a ^= b;
    b ^= a;
    a ^= b;
    println!("{} {}", a, b);
}
