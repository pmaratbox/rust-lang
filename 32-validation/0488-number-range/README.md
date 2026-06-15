# 0488 — Number range

Use the `validator` crate with `#[derive(Validate)]` to enforce that the `age` field satisfies `range(min = 0, max = 120)`. Validating `{name: "alice", age: 200}` violates the range constraint, so `validate()` returns `Err`. The output is the sorted, lowercased failing field name(s) extracted from `e.field_errors().keys()` (here `age`), or `ok` if validation passes — never the library's message text.

## Run

    cargo run
