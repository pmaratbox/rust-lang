# 0031 — Type Conversion & Parsing

Parse the string `"42"` into an integer and `"3.5"` into a float, then convert the integer back to a string, printing `int: 42`, `float: 3.5`, and `str: 42`. `str::parse` is generic over the target type, inferred here from the `i32`/`f64` annotations; it returns a `Result`, unwrapped for brevity. `to_string` (via the `ToString`/`Display` traits) converts back.

## Run

    cargo run
