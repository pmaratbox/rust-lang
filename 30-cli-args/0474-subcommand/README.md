# 0474 — Subcommand

Use the `clap` crate (with its `derive` feature) to dispatch to a subcommand. A `Cli` struct derives `Parser` and holds a `Command` enum that derives `Subcommand`; the `add` variant carries two integer positionals. We call `Cli::parse_from(["prog", "add", "2", "3"])` on a fixed argv (instead of the real process args) so the output is deterministic, then match on the parsed subcommand to sum the two numbers.

## Run

    cargo run
