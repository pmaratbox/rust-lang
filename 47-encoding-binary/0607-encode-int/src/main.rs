fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

fn main() {
    // MessagePack-encode the integer 42. Small non-negative integers fit in a
    // single "positive fixint" byte, so the encoding is just 0x2a.
    let bytes = rmp_serde::to_vec(&42i32).unwrap();
    println!("{}", hexs(&bytes));
}
