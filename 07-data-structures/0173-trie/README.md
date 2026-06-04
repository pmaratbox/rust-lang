# 0173 — Trie

Insert "cat" and "car" into a trie, then search "car" (yes) and "can" (no), printing `yes no`. Each node holds a `HashMap<char, Trie>` of children plus an `end` flag for word boundaries.

## Run

    cargo run
