# 0504 — Error handler

Use the **axum** web framework with `tower-http`'s `CatchPanicLayer` error-handling middleware: the `GET /boom` handler throws a real error (`panic!`), and the layer catches it and turns it into an HTTP `500 Internal Server Error`. The route is exercised IN-PROCESS via tower's `ServiceExt::oneshot` (no bound port), and the program prints the real status code read from the framework's HTTP response with `res.status()`.

## Run

    cargo run
