use uuid::Uuid;

fn main() {
    // Parse an UPPERCASE UUID string with the `uuid` crate, then print its
    // canonical lowercase form. parse_str accepts any case; Display is canonical.
    let parsed = Uuid::parse_str("550E8400-E29B-41D4-A716-446655440000").unwrap();
    println!("{}", parsed);
}
