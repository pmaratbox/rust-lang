# 0090 — Binary Search Tree

Insert `5, 3, 8, 1, 4` into a binary search tree and print an in-order traversal (which yields the values in sorted order): `1 3 4 5 8`. Each link is an `Option<Box<Node>>`; `insert` takes ownership (using `take()` to move the child out) and in-order traversal borrows the tree to collect values.

## Run

    cargo run
