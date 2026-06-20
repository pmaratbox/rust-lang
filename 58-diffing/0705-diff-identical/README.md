# 0705 — Identical inputs

Uses the `similar` crate's `TextDiff::from_lines` to diff `A` against
itself (`A -> A`). Every change carries a `ChangeTag`; with identical
inputs the LCS-based diff yields only `Equal` tags, so the counts of
`Insert` (added) and `Delete` (removed) lines are both zero. The result
is computed from the real diff, never hardcoded.

## Run

    cargo run
