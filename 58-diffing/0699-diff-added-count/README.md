# 0699 — Count added lines

Uses the `similar` crate's `TextDiff::from_lines` to diff
`A=[apple, banana, cherry]` against `B=[apple, blueberry, cherry, date]`.
Each change carries a `ChangeTag`; counting the `Insert` tags yields the
number of ADDED lines, computed from the real LCS-based diff (never
hardcoded).

## Run

    cargo run
