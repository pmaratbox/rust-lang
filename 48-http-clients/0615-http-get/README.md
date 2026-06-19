# 0615 — GET request

Uses the `reqwest` blocking HTTP client to make a `GET /hello` request to an
in-process `tiny_http` server bound to an ephemeral loopback port
(`127.0.0.1:0`, never printed). The server returns the fixed text body
`hello world`, and the client reads it back with `.text()` and prints it.

## Run

    cargo run
