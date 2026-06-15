# 0490 — Email format

Use the `validator` crate's `#[derive(Validate)]` to declare a schema on a struct: the `email` field carries the `#[validate(email)]` constraint, so it must be a syntactically valid email address. Validating the input `{ email: "not-an-email" }` fails, and the program prints the sorted, lowercased failing field name(s) extracted from `field_errors().keys()` — here `email`. On success it would print `ok`.

## Run

    cargo run
