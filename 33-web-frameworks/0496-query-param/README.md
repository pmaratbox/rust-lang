# 0496 — Query parameter

Defines a GET `/greet` route on an `axum` `Router` whose handler reads the `name` query-string parameter via axum's `Query` extractor and returns `hello ` + the name, then exercises it in-process using `tower`'s `ServiceExt::oneshot` (no port is bound) and prints the body read from the real HTTP response: `hello alice`.

## Run

    cargo run
