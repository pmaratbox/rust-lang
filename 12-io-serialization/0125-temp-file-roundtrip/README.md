# 0125 — Temp File Roundtrip

Write a string to a temporary file, read it back, confirm it matches, delete the file, and print `roundtrip: ok`. `env::temp_dir()` locates the system temp directory for a portable scratch path.

## Run

    cargo run
