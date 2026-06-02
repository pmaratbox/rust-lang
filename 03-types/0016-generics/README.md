# 0016 — Generics

Define a generic `first` function that returns the first element of a list, then call it on a list of integers and a list of strings to show one definition working at two types. Rust writes `fn first<T>(items: &[T]) -> &T` and *monomorphizes* it — the compiler generates a specialized copy for each concrete `T` (here `i32` and `&str`) at compile time, so there is no runtime cost. Returning `&T` borrows from the slice and needs no trait bound; printing the result with `{}` relies on `T: Display`, which both types satisfy.

## Run

    cargo run
