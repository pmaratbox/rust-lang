# 0503 — Method routing

This lesson uses the **axum** web framework to bind two HTTP methods to one path: on `/item`, `GET` returns `get` and `POST` returns `post`, chained as `get(...).post(...)` on a single `Router` route. It then exercises `POST /item` **in-process** via `tower::ServiceExt::oneshot` (no port is bound), reading the actual HTTP response body with `axum::body::to_bytes` and printing it.

## Run

    cargo run
