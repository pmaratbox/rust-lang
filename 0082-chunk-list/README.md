# 0082 — Chunk a List

Split the list `1, 2, 3, 4, 5, 6, 7` into chunks of `3` and print each chunk on its own line: `1 2 3`, `4 5 6`, `7`. `slice::chunks(3)` yields subslices of up to three elements, the last shorter if the length is not a multiple.

## Run

    cargo run
