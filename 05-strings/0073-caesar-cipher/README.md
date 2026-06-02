# 0073 — Caesar Cipher

Encrypt `abc` with a Caesar cipher shifting each letter forward by `1` (wrapping within the alphabet) and print the result: `bcd`. `bytes()` yields `u8`; `b'a'` is the byte literal for `a`, and `% 26` wraps the shift before casting back to `char`.

## Run

    cargo run
