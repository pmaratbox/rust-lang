# 0007 — Collections

Build an array of the integers `1, 2, 3, 4, 5`, then print its count and its
first and last elements. `[1, 2, 3, 4, 5]` is a fixed-size array `[i32; 5]`
and `.len()` gives its length. Direct `nums[i]` panics on out-of-bounds; the
safe idiom is `.first()` / `.last()`, which return `Option<&T>`. `Vec<T>` is
the growable counterpart.

## Run

    cargo run
