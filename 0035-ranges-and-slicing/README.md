# 0035 — Ranges & Slicing

From the list `[10, 20, 30, 40, 50]`, take the sub-sequence at indices 1 through 4 (exclusive) and print `slice: 20 30 40`. `&nums[1..4]` borrows a half-open *slice* — a view (pointer + length) into the array, not a copy. Range forms include `1..=3` (inclusive), `..3`, and `1..`; an out-of-bounds range panics.

## Run

    cargo run
