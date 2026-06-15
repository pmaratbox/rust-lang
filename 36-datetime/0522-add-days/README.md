# 0522 — Add days

Uses the `chrono` crate to parse a fixed ISO date into a `NaiveDate`, then adds
10 days with `Duration::days` (chrono's duration arithmetic) and formats the
result back to ISO 8601 with `%Y-%m-%d`.

## Run

    cargo run
