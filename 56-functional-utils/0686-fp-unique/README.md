# 0686 — Unique

Uses Rust's `itertools` crate and its `.unique` iterator adaptor to remove duplicates from `[1, 2, 2, 3, 3, 3]` while preserving first-seen order, then joins the deduplicated values into a comma-separated string with itertools' `.join`.

## Run

    cargo run
