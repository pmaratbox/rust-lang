# 0196 — Error Wrapping

Wrap an inner error "inner" inside an outer context and print the combined message `outer: inner`. Rust wraps errors by holding the inner one as a `source()` and rendering both through `Display`.

## Run

    cargo run
