use base64::{engine::general_purpose::STANDARD, Engine};

fn main() {
    let encoded = STANDARD.encode(b"hello");
    println!("{}", encoded);
}
