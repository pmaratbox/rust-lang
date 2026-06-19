# 0619 — Query parameters

Uses the `reqwest` blocking HTTP client to send a `GET /greet` request with the
query parameter `name=Bob` (built via `.query(&[("name", "Bob")])`) to an
in-process `tiny_http` server bound to an ephemeral loopback port
(`127.0.0.1:0`, never printed). The server reads the `name` query parameter and
returns `hi <name>`; the client reads the body with `.text()` and prints it,
yielding `hi Bob`.

## Run

    cargo run
