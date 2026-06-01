# 0039 — Records & Value Equality

Create two points with the same fields, print one as `point: (1, 2)`, and compare them by value to print `equal: yes`. `#[derive(PartialEq)]` generates a field-wise equality so `==` works; `#[derive(Eq)]` adds total equality. Without the derive, the type cannot be compared at all — equality is opt-in.

## Run

    cargo run
