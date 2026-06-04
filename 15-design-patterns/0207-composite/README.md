# 0207 — Composite

Sum a composite tree of leaf values 1, 2, 3 through a uniform size() interface, printing `6`. A `Component` trait is implemented by both `Leaf` and `Composite` holding `Box<dyn Component>` children.

## Run

    cargo run
