# 0684 — Chunk

Uses Rust's `itertools` crate and its `.chunks` adaptor to split `[1, 2, 3, 4, 5, 6]` into fixed-size pieces of 2. Each chunk is comma-joined and the chunks are joined with `|` using itertools' `.join`.

## Run

    cargo run
