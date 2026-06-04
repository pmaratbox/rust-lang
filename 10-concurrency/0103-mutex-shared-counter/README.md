# 0103 — Mutex-Protected Counter

Have multiple threads each increment a shared counter under a mutex so the total is exactly `1000`. An `Arc<Mutex<T>>` shares ownership across threads while `lock()` serializes every increment.

## Run

    cargo run
