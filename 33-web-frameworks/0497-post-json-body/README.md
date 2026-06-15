# 0497 — POST JSON body

Use the `axum` web framework's `Json` extractor to parse a JSON request body on `POST /sum`. The handler deserializes the body `{"a":2,"b":3}` into a `#[derive(Deserialize)]` struct and returns the sum. The route is exercised in-process with `tower::ServiceExt::oneshot` (no port is bound), and the response body read via `axum::body::to_bytes` is printed, yielding `5`.

## Run

    cargo run
