# 0616 — Response status code

Uses the `reqwest` blocking HTTP client to call an in-process `tiny_http`
server bound to `127.0.0.1:0` (an ephemeral loopback port, never printed).
The route `GET /hello` returns `200`; the client reads the response's
status code and prints it as an integer: `200`.

## Run

    cargo run
