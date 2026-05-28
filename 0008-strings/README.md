# 0008 — Strings

Given `name = "world"`, print a greeting, the name in uppercase, and its
length. `.to_uppercase()` returns a new `String`. `.len()` returns the number
of **bytes** in the UTF-8 encoding, not characters — use `.chars().count()`
for the code-point count. For ASCII they match. The literal `"world"` is a
`&str` (a borrowed string slice).

## Run

    cargo run
