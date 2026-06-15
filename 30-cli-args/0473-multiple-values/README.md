# 0473 — Multiple values

Use the `clap` crate with its `derive` feature to collect a repeated option into a list. Declaring the field as a `Vec<i64>` makes `--num` repeatable, so clap appends each occurrence into the vector. The parser runs over a fixed hardcoded argv (`["prog", "--num", "1", "--num", "2", "--num", "3"]`) instead of the real process args, so the output is deterministic; the three values are summed and printed.

## Run

    cargo run
