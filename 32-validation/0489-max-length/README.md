# 0489 — Maximum length

Uses the real [`validator`](https://crates.io/crates/validator) crate with a
`#[derive(Validate)]` struct. The `code` field carries a
`#[validate(length(max = 5))]` constraint, so the seven-character value
`"ABCDEFG"` is too long. The program prints the failing field name(s) — the
sorted, lowercased keys of `e.field_errors()` — or `ok` when validation passes.

## Run

    cargo run
