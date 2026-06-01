# 0053 — Binary Search

Binary-search the sorted array `1, 3, 5, 7, 9` for `7` and print the index where it is found: `found 7 at index 3`. The indices are `i32` so `hi` can reach `-1` when the value is absent; the slice is indexed with `mid as usize`. The stdlib `slice::binary_search` returns `Result<usize, usize>`.

## Run

    cargo run
