# 0636 — Boolean value

Uses the `config` crate to build a layered configuration from `config.json`
(plus a `missing` default and an `APP_`-prefixed environment source). The
boolean key `debug` is read with `get_bool` and printed, so Rust's `bool`
`Display` renders it lowercase as `true`.

## Run

    cargo run
