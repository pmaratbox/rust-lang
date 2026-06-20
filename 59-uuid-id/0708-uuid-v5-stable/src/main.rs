use uuid::Uuid;

fn main() {
    // UUIDv5 (name-based, SHA-1) is deterministic from (namespace, name).
    let a = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"example.com");
    let b = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"example.com");
    println!("{}", a == b);
}
