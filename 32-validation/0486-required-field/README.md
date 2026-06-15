# 0486 — Required field

Use the `validator` crate to enforce a required-field constraint. The schema requires both `name` and `age`; each is an `Option` checked by a `#[validate(custom(...))]` function that fails when the value is `None`. The input has `name` present but `age` missing, so validation fails. The output is the failing field name(s) extracted from `field_errors().keys()` (lowercased, sorted, one per line) — never library-specific message text.

## Run

    cargo run
