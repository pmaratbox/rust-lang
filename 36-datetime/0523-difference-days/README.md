# 0523 — Difference in days

Uses the `chrono` crate to parse two fixed ISO dates into `NaiveDate` values,
then computes the span between them with `signed_duration_since` (chrono's
`Duration`) and reports the whole-day count via `num_days()`.

## Run

    cargo run
