# 0027 — File I/O

Write `hello, file` to a file, read it back, delete the file, and print `read: hello, file`. `std::fs::write` and `std::fs::read_to_string` are one-call helpers that open, transfer, and close the file, returning a `Result` (`.unwrap()` here panics on error); `fs::remove_file` deletes it. A file handle closes when it drops.

## Run

    cargo run
