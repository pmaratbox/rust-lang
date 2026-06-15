# 0464 — Optional field default

Deserialize JSON that is missing a field using the `serde` framework with `serde_json`. The input `{"name":"alice"}` has no `age` key, but the `Person` struct marks that field `#[serde(default)]`, so serde supplies the type's `Default` value (`i32` -> `0`) instead of failing. The result `alice 0` is printed.

## Run

    cargo run
