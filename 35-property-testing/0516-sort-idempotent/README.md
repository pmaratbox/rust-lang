# 0516 — Sort is idempotent

This lesson uses the `proptest` property-testing library driven programmatically
via `TestRunner`. The `proptest::collection::vec` generator produces random
integer lists, and the property `sort(sort(xs)) == sort(xs)` is checked over
~100 generated cases; passing prints `passed`.

## Run

    cargo run
