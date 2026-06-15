use chrono::{Datelike, NaiveDate};

fn main() {
    // Fixed date (never the current time); parsed by chrono from an ISO string.
    let d = NaiveDate::parse_from_str("2026-06-15", "%Y-%m-%d").unwrap();

    // Extract components via the Datelike accessors.
    println!("{}", d.year());
    println!("{}", d.month());
    println!("{}", d.day());
}
