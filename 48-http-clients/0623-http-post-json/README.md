# 0623 — POST JSON, parse JSON

Uses the `reqwest` blocking HTTP client to POST a JSON body to an
in-process `tiny_http` server bound to an ephemeral loopback port
(`127.0.0.1:0`). The route `POST /double` reads `{"x":N}` and replies
`{"doubled":2N}`; the client sends `{"x":5}`, parses the JSON reply with
`serde_json`, and prints the doubled value: `10`.

## Run

    cargo run
