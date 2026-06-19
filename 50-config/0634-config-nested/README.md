# 0634 — Nested key

Uses the `config` crate to load `config.json` via `Config::builder()` with a
`File` source (plus a default and an `Environment` prefix source for parity with
the rest of the series). The nested key `server.port` is resolved with the
crate's dotted-path accessor `get_int("server.port")` and printed as an integer,
producing `8080`.

## Run

    cargo run
