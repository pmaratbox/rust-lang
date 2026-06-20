# 0701 — Added line content

Uses Rust's `similar` crate (`TextDiff::from_lines`) to diff two fixed line-lists A and B. It walks `iter_all_changes()` and collects every `ChangeTag::Insert` line — the lines present in B but not in A — then prints them in document (B) order, comma-joined. The added set is LCS-determined, so the result is deterministic regardless of edit-script ordering.

## Run

    cargo run
