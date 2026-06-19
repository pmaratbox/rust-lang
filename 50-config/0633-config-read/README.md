# 0633 — Read a value

Uses the `config` crate to load `config.json` via a `Config::builder()` with a
`File` source (plus an `Environment` source with prefix `APP` for later
override lessons). The top-level string key `name` is resolved with
`get_string("name")` and printed: `myapp`.

## Run

    cargo run
