fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

fn main() {
    // MessagePack-encode the boolean `true`. Booleans have dedicated single-byte
    // type tags, so `true` encodes to 0xc3 (and `false` would be 0xc2).
    let bytes = rmp_serde::to_vec(&true).unwrap();
    println!("{}", hexs(&bytes));
}
