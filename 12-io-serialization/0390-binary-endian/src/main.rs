fn main() {
    let n: u16 = 258;
    let bytes = n.to_be_bytes();
    let decoded = u16::from_be_bytes(bytes);
    println!("{} {} {}", bytes[0], bytes[1], decoded);
}
