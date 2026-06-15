# 0472 — Default value

Parse command-line arguments with the `clap` crate (derive API). The `--count` option is declared with `#[arg(long, default_value_t = 1)]`, so when it is absent from the argv clap supplies the default `1` instead of erroring. To stay deterministic, the program parses a hardcoded empty argv (`["prog"]`) via `Args::parse_from` rather than the real process arguments, so it always prints `1`.

## Run

    cargo run
