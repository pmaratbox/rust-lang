# 0617 — Parse JSON response

Uses the `reqwest` blocking HTTP client to make a `GET /user` request to an
in-process `tiny_http` server bound to an ephemeral loopback port
(`127.0.0.1:0`, never printed). The server returns the fixed JSON body
`{"name":"Alice","age":30}`; the client parses it with `.json()` into a
`serde_json::Value` and prints the `name` field: `Alice`.

## Run

    cargo run
