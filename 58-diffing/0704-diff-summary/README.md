# 0704 — Diff summary

Uses the `similar` crate's `TextDiff::from_lines` to diff
`A=[apple, banana, cherry]` against `B=[apple, blueberry, cherry, date]`.
Each change carries a `ChangeTag`; tallying `Insert`, `Delete`, and `Equal`
tags yields the added, removed, and unchanged counts, printed space-joined.
All counts come from the real LCS-based diff (never hardcoded).

## Run

    cargo run
