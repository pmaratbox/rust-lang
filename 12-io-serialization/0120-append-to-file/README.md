# 0120 — Append to a File

Write "a" to a file, append "b", then read both lines back and print `a b`. `OpenOptions::new().append(true)` reopens the file without truncating it.

## Run

    cargo run
