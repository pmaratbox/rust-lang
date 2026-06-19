# 0621 — Read a response header

Uses the `reqwest` blocking HTTP client to make a `GET /info` request to an
in-process `tiny_http` server bound to an ephemeral loopback port
(`127.0.0.1:0`, never printed). The server sets a custom response header
`X-Count: 7`, and the client reads it back from `response.headers()` and prints
the value.

## Run

    cargo run
