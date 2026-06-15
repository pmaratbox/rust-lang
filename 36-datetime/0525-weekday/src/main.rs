use chrono::{Datelike, NaiveDate};

fn main() {
    // Fixed date (never the current time): parse an ISO date string.
    let d = NaiveDate::parse_from_str("2026-06-15", "%Y-%m-%d").unwrap();

    // chrono's number_from_monday() is already ISO: Monday = 1 .. Sunday = 7.
    let iso_weekday = d.weekday().number_from_monday();

    println!("{}", iso_weekday);
}
