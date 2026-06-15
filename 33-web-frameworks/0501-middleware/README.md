# 0501 — Middleware

Using the **axum** web framework, attach a response-transforming **middleware** via `axum::middleware::from_fn`. The route `GET /` handler returns only `hello`; the middleware runs the inner handler with `next.run(req)`, reads the response body, and prefixes it with `[mw] `, yielding `[mw] hello`. The route is exercised **in-process** with tower's `ServiceExt::oneshot` (no fixed port), and the real HTTP response body is printed.

## Run

    cargo run
