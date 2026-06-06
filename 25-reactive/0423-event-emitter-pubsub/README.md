# 0423 — EventEmitter (Pub/Sub)

Build a multi-topic EventEmitter with on(topic, handler), emit(topic, payload), and off(topic, handler). In Rust, handlers are stored as `Box<dyn Fn(&str)>` boxed closures keyed by topic in a `HashMap`, with `Vec::retain` removing a tagged subscription on `off`.

## Run

    cargo run
