# 0403 — Subscribe and Unsubscribe

Return a Subscription from subscribe() and use it to unsubscribe so later values are not delivered. In Rust a shared `Rc<Cell<bool>>` closed flag lets `unsubscribe()` signal the producer to stop before the next push.

## Run

    cargo run
