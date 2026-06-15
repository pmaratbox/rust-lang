# 0533 — Map & filter

The `im` crate's persistent `Vector` transforms via `filter` and `map`, each returning a brand-new immutable collection while the original list `[1, 2, 3, 4, 5]` stays unchanged. Here we keep the even elements then multiply each by 10, yielding `20 40`.

## Run

    cargo run
