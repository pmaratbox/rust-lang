use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

fn main() {
    let mut mac = HmacSha256::new_from_slice(b"key").expect("HMAC accepts any key length");
    mac.update(b"hello");
    let tag = mac.finalize().into_bytes();
    let hex = tag.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("{}", hex);
}
