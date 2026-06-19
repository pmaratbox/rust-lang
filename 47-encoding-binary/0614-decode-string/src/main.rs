// Rust — rmp-serde. Per-lesson Cargo. Run: cargo run
// DECODE: parse the hex string into bytes, then decode with the MessagePack library.

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn main() {
    let hex = "a568656c6c6f";
    let bytes = hex_to_bytes(hex);
    let s: String = rmp_serde::from_slice(&bytes).unwrap();
    println!("{}", s); // hello
}
