# 0024 — Sorting & Comparators

Sort `[3, 1, 2]` ascending, then again with a custom comparator that reverses the order, printing `asc: 1 2 3` and `desc: 3 2 1`. `sort` orders in place by `Ord`; `sort_by` takes a closure returning an `Ordering`, and `b.cmp(a)` reverses it. These are stable sorts (`sort_unstable` trades stability for speed), and the vector must be `mut`.

## Run

    cargo run
