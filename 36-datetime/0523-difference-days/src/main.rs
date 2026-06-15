use chrono::NaiveDate;

fn main() {
    // Two fixed ISO dates (never the current time), parsed by chrono.
    let start = NaiveDate::parse_from_str("2026-06-15", "%Y-%m-%d").unwrap();
    let end = NaiveDate::parse_from_str("2026-07-15", "%Y-%m-%d").unwrap();

    // Difference computed by the library via signed_duration_since.
    let diff = end.signed_duration_since(start);

    // Whole days between the two dates.
    println!("{}", diff.num_days());
}
