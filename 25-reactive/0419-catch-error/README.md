# 0419 — Catch Error

Implement catchError that, on an error from the source, switches to a fallback stream. Observer callbacks are shared as `Rc<RefCell<dyn FnMut>>` so the error handler can re-subscribe the same downstream observer to the fallback.

## Run

    cargo run
