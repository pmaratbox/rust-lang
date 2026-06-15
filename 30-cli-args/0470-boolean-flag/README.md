# 0470 — Boolean flag

Use the `clap` crate with its `derive` feature to parse a boolean flag. The `Args` struct declares a `--verbose` flag as a `bool` field; clap sets it to `true` when present. To stay deterministic, the program parses a fixed argv `["prog", "--verbose"]` via `Args::parse_from` instead of reading the real process arguments, then prints the flag value as lowercase `true`.

## Run

    cargo run
