# 0706 — All lines added

Uses the `similar` crate's `TextDiff::from_lines` to diff an EMPTY list
against `[x, y]`. Because the source side is empty, the real LCS-based
diff marks every line in the target with the `Insert` `ChangeTag`, so
counting those tags gives the ADDED count (never hardcoded).

## Run

    cargo run
