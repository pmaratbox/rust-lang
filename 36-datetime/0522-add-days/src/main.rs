use chrono::{Duration, NaiveDate};

fn main() {
    // Fixed date (never the current time), parsed by chrono.
    let start = NaiveDate::parse_from_str("2026-06-15", "%Y-%m-%d").unwrap();

    // Add days using the library's duration arithmetic.
    let result = start + Duration::days(10);

    // Format back to ISO 8601.
    println!("{}", result.format("%Y-%m-%d"));
}
