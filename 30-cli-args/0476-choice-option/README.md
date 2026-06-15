# 0476 — Choice option

Use `clap` with its `derive` feature to restrict an option to a fixed set of choices. A `Color` enum derives `ValueEnum`, so `--color` only accepts `red`, `green`, or `blue`; clap rejects anything else. For deterministic output the program parses a fixed argv `["prog", "--color", "green"]` via `Args::parse_from` instead of the real process arguments, then prints the chosen value (`green`).

## Run

    cargo run
