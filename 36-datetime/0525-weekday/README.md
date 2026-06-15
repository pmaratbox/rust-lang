# 0525 — Weekday

Uses the `chrono` crate: `NaiveDate::parse_from_str` parses the fixed ISO date `2026-06-15`, then `.weekday().number_from_monday()` returns its ISO weekday number (Monday = 1 .. Sunday = 7). June 15, 2026 is a Monday, so the program prints `1`.

## Run

    cargo run
