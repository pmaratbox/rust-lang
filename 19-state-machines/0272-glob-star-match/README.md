# 0272 — Glob Star Match

Match the glob "a*b" (* = any run) against "aaab" (yes) and "aac" (no), printing `yes no`. Recursive backtracking over byte slices implements `*` as zero-or-more characters.

## Run

    cargo run
