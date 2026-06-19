# 0640 — Read a section

Uses the `config` crate to load `config.json` via `Config::builder()` with a
`File` source (plus a default and an `Environment` prefix source for parity with
the rest of the series). Two keys from the `server` section are resolved with the
crate's dotted-path accessors — `get_string("server.host")` and
`get_int("server.port")` — and combined into `host:port`, producing
`0.0.0.0:8080`.

## Run

    cargo run
