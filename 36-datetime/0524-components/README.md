# 0524 — Date components

Uses the `chrono` crate to parse the fixed ISO date `2026-06-15` with `NaiveDate::parse_from_str`, then extracts each component through the `Datelike` accessors (`.year()`, `.month()`, `.day()`) and prints them on their own lines.

## Run

    cargo run
