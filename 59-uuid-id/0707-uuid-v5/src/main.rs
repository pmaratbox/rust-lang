use uuid::Uuid;

fn main() {
    // UUIDv5 is name-based (SHA-1): deterministic from (namespace, name).
    let id = Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"example.com");
    println!("{}", id);
}
