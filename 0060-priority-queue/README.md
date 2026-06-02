# 0060 — Priority Queue

Push `3`, `1`, and `2` into a min-priority-queue, then pop them all and print them in priority (ascending) order: `1 2 3`. `BinaryHeap` is a MAX-heap, so wrapping each value in `std::cmp::Reverse` flips the ordering to pop the minimum first.

## Run

    cargo run
