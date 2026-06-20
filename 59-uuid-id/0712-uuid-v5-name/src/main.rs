use uuid::Uuid;

fn main() {
    // UUIDv5 is SHA-1 name-based: deterministic from (namespace, name).
    // A different name than example.com yields a different UUID.
    println!("{}", Uuid::new_v5(&Uuid::NAMESPACE_DNS, b"test.com"));
}
