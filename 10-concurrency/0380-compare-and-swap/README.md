# 0380 — Compare-And-Swap Loop

Increment a shared value to 100 using a CAS retry loop from multiple threads, printing `100`. Rust's `AtomicUsize::compare_exchange` retries until the load matches, guaranteeing no lost updates.

## Run

    cargo run
