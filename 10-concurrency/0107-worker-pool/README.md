# 0107 — Worker Pool

Distribute squaring of 1..4 across a pool of workers, collect the results, and print them sorted ascending `1 4 9 16`. A shared `Arc<Mutex<Receiver>>` feeds jobs to several worker threads that return squares over a results channel.

## Run

    cargo run
