# 0618 — POST a body

Uses the `reqwest` blocking HTTP client to send a `POST /echo` request with the
text body `ping` to an in-process `tiny_http` server bound to an ephemeral
loopback port (`127.0.0.1:0`, never printed). The server reads the request body
and returns it verbatim; the client reads the response with `.text()` and prints
it, yielding `ping`.

## Run

    cargo run
