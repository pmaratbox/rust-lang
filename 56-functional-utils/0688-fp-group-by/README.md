# 0688 — Group by

Uses Rust's `itertools` crate and its `into_group_map_by` transform to group `[1, 2, 3, 4, 5, 6]` by parity into a map of key to values, then sorts the keys (`even` before `odd`) and renders each group as `key:v1,v2,...` joined by `;`, yielding `even:2,4,6;odd:1,3,5`.

## Run

    cargo run
