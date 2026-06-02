# 0074 — Run-Length Encoding

Run-length encode the string `aaabbc` (each run of a repeated character becomes the character followed by its count), printing `a3b2c1`. Collecting to `Vec<char>` allows indexing; the inner loop counts a run, then `push`/`push_str` append the character and its count.

## Run

    cargo run
