# 0475 — Required option

Use the `clap` crate (with its `derive` feature) to declare a required option. The `--id` integer option is marked `required = true` so parsing fails if it is absent. The program parses a fixed argv `["prog", "--id", "42"]` instead of the real process arguments, keeping the output deterministic, and prints the parsed value `42`.

## Run

    cargo run
