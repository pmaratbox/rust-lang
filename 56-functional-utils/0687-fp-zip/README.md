# 0687 — Zip

Uses Rust's `itertools` crate together with the standard iterator's `zip` transform to pair `[1, 2, 3]` element-wise with `["a", "b", "c"]`, formatting each pair as `<n><s>` and comma-joining the results into `1a,2b,3c`.

## Run

    cargo run
