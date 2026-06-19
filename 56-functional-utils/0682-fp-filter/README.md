# 0682 — Filter

Using the `itertools` crate (on top of the std iterator adapters), we take the
range `[1,2,3,4,5,6]`, apply the std `filter` transform with an even-number
predicate, and use itertools' `.join` to comma-join the survivors into `2,4,6`.

## Run

    cargo run
