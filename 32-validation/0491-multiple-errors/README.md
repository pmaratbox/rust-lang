# 0491 — Multiple errors

Use the `validator` crate with `#[derive(Validate)]` to check several constraints at once: `name` must have `length(min = 3)` and `age` must fall in `range(min = 0, max = 120)`. The input `{ name: "al", age: 200 }` breaks both rules. Because `validate()` aggregates every failure rather than stopping at the first, `ValidationErrors::field_errors()` exposes a map keyed by field name. The output is the failing field name(s) pulled from that error object, lowercased and sorted — here `age` and `name` — or `ok` if validation passes.

## Run

    cargo run
