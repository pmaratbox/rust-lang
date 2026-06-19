# 0622 — Handle 404

Uses the `reqwest` blocking HTTP client to call an in-process `tiny_http`
server bound to `127.0.0.1:0` (an ephemeral loopback port, never printed).
The server defines no route, so a request to `GET /missing` returns a 404;
the client reads the response's status code and prints it as an integer: `404`.

## Run

    cargo run
