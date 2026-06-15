# 0469 — Positional argument

Use the `clap` crate with its `derive` feature to define a CLI with a single positional argument `name`. A bare `String` field on the `#[derive(Parser)]` struct becomes a positional argument (no `--flag`). For determinism the program parses a fixed argv `["prog", "alice"]` via `Args::parse_from` instead of the real process args, so it always prints `alice`.

## Run

    cargo run
