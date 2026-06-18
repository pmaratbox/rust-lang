# 0568 — Nested YAML mapping

Use the `serde_yaml` crate to parse a fixed YAML document that contains a
nested `server` mapping (`host` and `port`). Read `server.host` and
`server.port` from the parsed `serde_yaml::Value` tree and print them in
the controlled `host:port` form.

## Run

    cargo run
