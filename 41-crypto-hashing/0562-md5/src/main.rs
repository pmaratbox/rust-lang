use md5::{Digest, Md5};

fn main() {
    let mut hasher = Md5::new();
    hasher.update(b"hello");
    let digest = hasher.finalize();
    println!(
        "{}",
        digest.iter().map(|b| format!("{:02x}", b)).collect::<String>()
    );
}
