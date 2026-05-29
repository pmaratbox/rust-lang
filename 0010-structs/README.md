# 0010 — Structs

Define a `Person` struct with a `name` and an `age`, create one ("Ada", 36),
and print each field. `struct Person { ... }` declares named fields; a struct
literal `Person { name: ..., age: ... }` builds an instance. `String::from`
makes an owned string for the `String` field (a bare `&str` literal wouldn't
match the type).

## Run

    cargo run
