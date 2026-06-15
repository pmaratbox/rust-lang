use chrono::NaiveDate;

fn main() {
    // Parse a FIXED ISO date string with chrono, then format it back to ISO.
    let date = NaiveDate::parse_from_str("2026-06-15", "%Y-%m-%d").unwrap();
    println!("{}", date.format("%Y-%m-%d"));
}
