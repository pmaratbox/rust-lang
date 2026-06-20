# 0703 — Count unchanged lines

Uses the `similar` crate's `TextDiff::from_lines` to diff
`A=[apple, banana, cherry]` against `B=[apple, blueberry, cherry, date]`.
Each change carries a `ChangeTag`; counting the `Equal` tags yields the
number of UNCHANGED lines (`apple`, `cherry`), computed from the real
LCS-based diff (never hardcoded).

## Run

    cargo run
