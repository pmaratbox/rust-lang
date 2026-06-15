# 0441 — Aggregate functions

Create an in-memory table, insert five integer amounts, then run a single `select count(*), sum(amount), min(amount), max(amount) from t` and print the four aggregate values, each on its own line. Uses the real `rusqlite` driver (bundled SQLite) with a prepared statement and `query_row` to read the aggregate row.

## Run

    cargo run
