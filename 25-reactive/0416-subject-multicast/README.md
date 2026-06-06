# 0416 — Subject Multicast

Implement a Subject that multicasts each emission to all current observers; two observers both receive 1 then 2. Idiomatic Rust stores observers as a `Vec<Box<dyn Fn(i32)>>` and iterates them in registration order on each `next`.

## Run

    cargo run
