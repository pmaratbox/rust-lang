# 0521 — Parse & format

Parse an ISO date and format it back. This lesson uses the `chrono` crate: `NaiveDate::parse_from_str` reads the fixed ISO date `2026-06-15` with the `%Y-%m-%d` pattern, then `NaiveDate::format` renders it back to ISO (yyyy-MM-dd).

## Run

    cargo run
