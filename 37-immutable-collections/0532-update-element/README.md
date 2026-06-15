# 0532 — Update an element

Uses the `im` crate's persistent `Vector`. Calling `a.update(0, 99)` returns a brand-new vector with index 0 replaced by `99` (sharing structure with the original), while the source vector `a` stays unchanged. The program prints the new vector `99 2 3` followed by the still-intact original `1 2 3`.

## Run

    cargo run
