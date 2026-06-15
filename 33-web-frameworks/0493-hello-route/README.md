# 0493 — Hello route

Defines a GET `/` route on an `axum` `Router` that returns the text `hello`, then exercises it in-process using `tower`'s `ServiceExt::oneshot` (no port is bound) and prints the body read from the real HTTP response: `hello`.

## Run

    cargo run
