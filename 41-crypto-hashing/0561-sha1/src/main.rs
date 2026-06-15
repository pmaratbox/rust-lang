use sha1::{Digest, Sha1};

fn main() {
    let mut hasher = Sha1::new();
    hasher.update(b"hello");
    let digest = hasher.finalize();
    println!(
        "{}",
        digest.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    );
}
