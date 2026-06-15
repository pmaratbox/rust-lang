# 0447 — Batch insert

Insert 1000 rows (values 1..1000) into an in-memory SQLite database efficiently using the `rusqlite` driver (with the `bundled` SQLite feature). All inserts run inside a single transaction created with `Connection::transaction`, reusing one prepared `insert` statement bound with a parameter for each row, then `select count(*)` reports the total (1000) via `query_row`.

## Run

    cargo run
