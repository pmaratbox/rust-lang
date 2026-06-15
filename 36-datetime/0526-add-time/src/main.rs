use chrono::{Duration, NaiveDateTime};

fn main() {
    // Parse a FIXED instant (never the current time).
    let start = NaiveDateTime::parse_from_str("2026-06-15T10:00", "%Y-%m-%dT%H:%M").unwrap();
    // Add 90 minutes via the library.
    let result = start + Duration::minutes(90);
    // Format as HH:mm.
    println!("{}", result.format("%H:%M"));
}
