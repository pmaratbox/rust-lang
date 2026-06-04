fn main() {
    let parity = |n: u32| n.count_ones() & 1;
    println!("{} {}", parity(7), parity(5));
}
