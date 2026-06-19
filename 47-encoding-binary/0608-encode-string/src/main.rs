fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

fn main() {
    // MessagePack-encode the string "hello". A short string is stored as a
    // "fixstr" header byte (0xa0 | len) followed by the UTF-8 bytes, so a
    // 5-byte string encodes to 0xa5 plus "hello".
    let bytes = rmp_serde::to_vec(&"hello").unwrap();
    println!("{}", hexs(&bytes));
}
