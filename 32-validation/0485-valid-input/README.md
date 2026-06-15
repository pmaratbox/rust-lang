# 0485 — Valid input

Use the `validator` crate's `#[derive(Validate)]` to declare a schema on a struct: `name` must satisfy `length(min = 3)` and `age` must satisfy `range(min = 0, max = 120)`. Calling `.validate()` on the valid input `{ name: "alice", age: 30 }` returns `Ok(())`, so the program prints `ok`. On failure it would instead print the sorted, lowercased failing field name(s) extracted from `field_errors().keys()`.

## Run

    cargo run
