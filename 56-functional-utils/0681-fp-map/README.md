# 0681 — Map

Uses Rust's `itertools` crate together with the standard iterator `map` adaptor to apply a pure function `x -> x*2` over `[1, 2, 3]`, then joins the transformed values into a comma-separated string with itertools' `.join`.

## Run

    cargo run
