use uuid::Uuid;

fn main() {
    // Parse a canonical UUID and read its version number via the uuid crate.
    let id = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap();
    println!("{}", id.get_version_num());
}
