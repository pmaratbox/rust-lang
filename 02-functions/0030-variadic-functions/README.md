# 0030 — Variadic Functions

Define a function that accepts a variable number of integer arguments and returns their total, then call it with `1, 2, 3` to print `sum: 6`. Rust has no variadic functions, so the idiomatic equivalent takes a slice `&[i32]` and the caller passes an array literal `&[1, 2, 3]`. Truly variadic call syntax exists only through macros like `println!`.

## Run

    cargo run
