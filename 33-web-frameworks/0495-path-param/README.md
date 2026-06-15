# 0495 — Path parameter

Use the `axum` web framework's `Path` extractor to capture the `{id}` segment of `GET /users/{id}`; the handler echoes the captured id back as the body. The route is exercised in-process with `tower::ServiceExt::oneshot` (no port is bound), and the response body read via `axum::body::to_bytes` is printed, yielding `42` for a request to `/users/42`.

## Run

    cargo run
