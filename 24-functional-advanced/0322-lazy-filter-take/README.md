# 0322 — Lazy Filter + Take

From a lazy stream of naturals, filter the even ones and take three, printing `2 4 6`. Chaining `.filter(..).take(3)` on the lazy `(1..)` range pulls only as many naturals as needed.

## Run

    cargo run
