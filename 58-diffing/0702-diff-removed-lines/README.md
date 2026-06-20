# 0702 — Removed line content

Uses Rust's `similar` crate (`TextDiff::from_lines`) to diff two fixed line-lists A and B. It walks `iter_all_changes()` and collects every `ChangeTag::Delete` line — the lines present in A but not in B — then prints them in document (A) order, comma-joined. The removed set is LCS-determined, so the result is deterministic regardless of edit-script ordering.

## Run

    cargo run
