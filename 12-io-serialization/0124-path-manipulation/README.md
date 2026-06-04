# 0124 — Path Manipulation

Join "/tmp" and "file.txt", then take the basename and extension, printing `/tmp/file.txt file.txt .txt`. `std::path::Path` provides `join`, `file_name`, and `extension` for portable path handling.

## Run

    cargo run
