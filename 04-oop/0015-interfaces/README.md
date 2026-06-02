# 0015 — Interfaces

Define a `Shape` interface with `name()` and `area()` methods, implement it for a rectangle and a square, then loop over a collection of shapes and print each one's area. Rust calls this a *trait*; `impl Shape for Rectangle` provides the methods for each type. To store mixed concrete types in one `Vec`, the elements are `Box<dyn Shape>` — trait objects that carry a vtable, so `s.area()` is *dynamic* dispatch. (Rust also offers static dispatch via generics `<T: Shape>` when the type is known at compile time.)

## Run

    cargo run
