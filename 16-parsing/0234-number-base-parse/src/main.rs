fn main() {
    let hex = i64::from_str_radix("ff", 16).unwrap();
    let bin = i64::from_str_radix("101", 2).unwrap();
    println!("{} {}", hex, bin);
}
