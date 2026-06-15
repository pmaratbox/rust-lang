# 0526 — Add time

Uses the `chrono` crate: `NaiveDateTime::parse_from_str` reads the fixed instant `2026-06-15T10:00`, then `Duration::minutes(90)` is added to it with the `+` operator to advance the time by 90 minutes. The resulting `NaiveDateTime` is rendered with `format("%H:%M")` to print `11:30`.

## Run

    cargo run
