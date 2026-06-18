# 0572 — TOML table

Use the `toml` crate to parse the fixed TOML document
`[server]\nhost = "localhost"\nport = 8080\n` into a `toml::Value`.
The `server.host` string is read with `as_str` and the `server.port`
integer with `as_integer`, then printed as `host=localhost port=8080`.

## Run

    cargo run
