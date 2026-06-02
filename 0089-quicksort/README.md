# 0089 — Quicksort

Sort the list `3, 1, 4, 1, 5, 2` using quicksort (partition around a pivot, then recurse on each side) and print the result: `1 1 2 3 4 5`. `filter().copied()` partitions the tail slice; the sorted `less` Vec gets the pivot pushed and the sorted `greater` extended onto it.

## Run

    cargo run
