# 0424 — Signal + Computed

Implement fine-grained reactivity: a writable signal and a derived computed that recomputes when its dependency changes. Uses `Rc<RefCell<_>>` for shared mutable state and `Rc<dyn Fn()>` subscriber callbacks.

## Run

    cargo run
