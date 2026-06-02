# 0028 — String Formatting

Format the float `3.14159` to two decimals and zero-pad the integer `42` to width five, printing `pi: 3.14` and `id: 00042`. Formatting macros use `{}` with a spec after the colon: `{:.2}` fixes two decimals and `{:05}` zero-pads to width 5. The format string is checked at compile time and is always locale-independent.

## Run

    cargo run
