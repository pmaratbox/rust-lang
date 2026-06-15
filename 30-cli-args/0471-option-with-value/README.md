# 0471 — Option with value

Define an option `--name` that takes a string value using the `clap` library with its `derive` feature. The `#[derive(Parser)]` macro turns a struct field into a `--name <value>` option, and `Args::parse_from` parses a fixed, hardcoded argv (`["prog", "--name", "alice"]`) rather than the real process arguments so the program is deterministic. The parsed value is then printed.

## Run

    cargo run
