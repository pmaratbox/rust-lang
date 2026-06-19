# 0624 — PUT request

Uses the `reqwest` blocking HTTP client to send a `PUT /item` request to an
in-process `tiny_http` server bound to an ephemeral loopback port
(`127.0.0.1:0`, never printed). The server matches the `PUT` method on `/item`
and returns the text `updated`; the client reads the response body with
`.text()` and prints it, yielding `updated`.

## Run

    cargo run
