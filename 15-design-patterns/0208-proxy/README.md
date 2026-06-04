# 0208 — Virtual Proxy

Use a lazy virtual proxy that loads the real subject only on first access, printing `loaded`. The proxy holds an `Option` initialized lazily on first `request()`.

## Run

    cargo run
