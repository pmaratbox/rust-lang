# 0499 — Custom status

Use the **axum** web framework to return a custom HTTP status code: the `POST /create` handler returns `StatusCode::CREATED` (201) instead of the default 200. The route is exercised IN-PROCESS via tower's `ServiceExt::oneshot` (no bound port), and the program prints the real status code read from the framework's HTTP response with `res.status()`.

## Run

    cargo run
