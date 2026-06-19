# 0620 — Send a request header

Uses the `reqwest` blocking HTTP client to call an in-process `tiny_http`
server bound to an ephemeral `127.0.0.1:0` loopback port (never printed).
The `GET /token` route echoes the request's `X-Token` header in the body;
the client sends `X-Token: secret` via `.header(...)`, so the response
body prints `secret`.

## Run

    cargo run
