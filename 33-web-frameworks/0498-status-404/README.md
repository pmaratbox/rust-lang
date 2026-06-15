# 0498 — 404 status

Build an `axum::Router` that only defines `GET /`, then exercise it IN-PROCESS with `tower::ServiceExt::oneshot` (no fixed port bound). Requesting the undefined route `GET /missing` makes axum's router return a real `404 Not Found` response; we read `res.status().as_u16()` straight off that HTTP response and print it, yielding `404`.

## Run

    cargo run
