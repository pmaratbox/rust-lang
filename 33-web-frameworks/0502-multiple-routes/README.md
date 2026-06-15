# 0502 — Multiple routes

This lesson uses the **axum** web framework to register two routes (`GET /` returning `home` and `GET /about` returning `about`) on a single `Router`, then exercises each one **in-process** via `tower::ServiceExt::oneshot`, reading the actual HTTP response body with `axum::body::to_bytes` — no port is bound. Each body is printed on its own line.

## Run

    cargo run
