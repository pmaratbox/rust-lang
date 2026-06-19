# 0639 — Integer value

Uses the `config` crate to load `config.json` via a `Config::builder()` with a
`File` source (plus an `Environment` source with prefix `APP`). The top-level
integer key `retries` is resolved with `get_int("retries")` and printed: `3`.

## Run

    cargo run
