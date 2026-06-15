use sha2::{Digest, Sha256};

fn main() {
    let mut hasher = Sha256::new();
    hasher.update(b"foo");
    hasher.update(b"bar");
    let digest = hasher.finalize();
    let hex = digest.iter().map(|b| format!("{:02x}", b)).collect::<String>();
    println!("{}", hex);
}
