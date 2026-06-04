fn main() {
    let x: u32 = 12;
    let lowest = x & x.wrapping_neg();
    println!("{}", lowest);
}
