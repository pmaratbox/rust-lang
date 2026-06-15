# 0500 — Request header

A handler on the `axum` web framework extracts the incoming `HeaderMap` and echoes the `X-Name` request header back as the response body. The route is exercised in-process via `tower::ServiceExt::oneshot`, building a `GET /whoami` request carrying `X-Name: alice` (no socket is bound, no fixed port). The response body is read with `axum::body::to_bytes` and printed, yielding `alice`.

## Run

    cargo run
