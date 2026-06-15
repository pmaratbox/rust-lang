use chrono::{DateTime, NaiveDateTime, Utc};

fn main() {
    // Parse a FIXED UTC instant (never the current time).
    let naive = NaiveDateTime::parse_from_str("2026-06-15T00:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let utc: DateTime<Utc> = naive.and_utc();

    // The library computes the Unix timestamp (epoch seconds).
    println!("{}", utc.timestamp());
}
