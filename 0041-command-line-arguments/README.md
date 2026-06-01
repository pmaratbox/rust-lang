# 0041 — Command-line Arguments

Read the first command-line argument and greet it, so running with `Ada` prints `hello, Ada`. `std::env::args()` is an iterator whose first item is the program name; `.nth(1)` skips it and returns the first real argument as an `Option`. `cargo run -- Ada` passes `Ada` past Cargo's own flags.

## Run

    cargo run -- Ada
