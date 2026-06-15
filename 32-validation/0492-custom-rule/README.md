# 0492 — Custom rule

Use the `validator` crate's custom-rule feature (`#[validate(custom(function = "..."))]`) to enforce a rule it has no built-in for: `password` must contain at least one digit. The custom function `has_digit` returns `Err(ValidationError)` when no digit is present. The fixed input `{ password: "abcdef" }` has no digit, so validation fails; the output is the failing field name(s) extracted from `e.field_errors()` (lowercased, sorted), here `password`.

## Run

    cargo run
