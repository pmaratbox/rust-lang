use pbkdf2::pbkdf2_hmac;
use sha2::Sha256;

fn main() {
    let mut out = [0u8; 32];
    pbkdf2_hmac::<Sha256>(b"password", b"salt", 1000, &mut out);
    let hex = out.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("{}", hex);
}
