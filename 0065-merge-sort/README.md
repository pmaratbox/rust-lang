# 0065 — Merge Sort

Sort the list `3, 1, 4, 1, 5, 2` using merge sort (recursively split in half, then merge the sorted halves) and print the result: `1 1 2 3 4 5`. `merge_sort` takes a slice and returns a sorted `Vec`; `merge` consumes both halves, pushing the smaller front value each step.

## Run

    cargo run
