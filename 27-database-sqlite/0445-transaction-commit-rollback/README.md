# 0445 — Transactions

Uses the real `rusqlite` SQLite driver against an in-memory database to demonstrate transaction control. It opens a transaction with `conn.transaction()`, inserts rows 1 and 2, and calls `tx.commit()`. A second transaction inserts 3 and then calls `tx.rollback()`, so that change is discarded. A final `select n from t order by n` confirms only the committed values 1 and 2 survive.

## Run

    cargo run
