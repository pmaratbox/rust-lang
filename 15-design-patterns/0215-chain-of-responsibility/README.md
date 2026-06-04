# 0215 — Chain of Responsibility

Pass a request of level 2 along a handler chain so the level-2 handler handles it, printing `handled by 2`. Handlers own an `Option<Box<dyn Handler>>` next link and forward when they cannot handle.

## Run

    cargo run
