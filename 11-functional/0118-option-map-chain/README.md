# 0118 — Option Map Chaining

Map a function over a present optional (10 -> 12) and an absent one (-> fallback), printing `12 none`. `Option::map` transforms a `Some` while threading `None` through untouched, and `unwrap_or_else` supplies the fallback string.

## Run

    cargo run
