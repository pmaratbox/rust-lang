# 0487 — Minimum length

Uses the `validator` crate's `#[validate(length(min = 3))]` constraint to check that `name` is at least 3 characters. The struct is derived with `#[derive(Validate)]`; on failure the program collects the failing field names from `field_errors()`, sorts them, and prints them (one per line) — or `ok` when validation passes. Here `{name:'al', age:30}` fails the minimum-length rule, so the output is the failing field name.

## Run

    cargo run
