# 0519 — Precondition / filter

This lesson uses the `proptest` property-testing library driven programmatically
via `TestRunner`. A `.prop_filter` precondition constrains the `any::<i32>()`
generator so only positive integers reach the property `n + 1 > n`, which is
checked over ~100 generated cases; passing prints `passed`.

## Run

    cargo run
