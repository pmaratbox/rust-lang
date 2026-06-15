# 0494 — JSON response

Use the `axum` web framework's `Json` responder to serialize a `#[derive(Serialize)]` struct into a JSON body for `GET /user`. The route is exercised in-process with `tower::ServiceExt::oneshot` (no port is bound), and the response body read via `axum::body::to_bytes` is printed, yielding the compact JSON `{"name":"alice"}`.

## Run

    cargo run
