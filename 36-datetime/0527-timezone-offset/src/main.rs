use chrono::{DateTime, FixedOffset, NaiveDateTime, Timelike, Utc};

fn main() {
    // Parse a FIXED UTC instant (never the current time).
    let naive = NaiveDateTime::parse_from_str("2026-06-15T12:00:00", "%Y-%m-%dT%H:%M:%S").unwrap();
    let utc: DateTime<Utc> = naive.and_utc();

    // Convert to a FIXED offset of +05:00 (no named tz / OS tzdata).
    let offset = FixedOffset::east_opt(5 * 3600).unwrap();
    let local: DateTime<FixedOffset> = utc.with_timezone(&offset);

    // The library computes the local hour: 12 + 5 = 17.
    println!("{}", local.hour());
}
