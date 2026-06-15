# 0448 — Upsert

Open an in-memory SQLite database with the `rusqlite` driver (using the `bundled` SQLite feature), create an `inv` table keyed by `item`, and insert `('apple', 5)`. It then runs an upsert with `insert ... on conflict(item) do update set qty = qty + excluded.qty`, which adds to the existing quantity for `apple` (bringing it to 10) and inserts `banana` as a new row. It uses `Connection::execute` with bound parameters for the inserts/upserts, then `prepare` plus `query_map` to read the rows ordered by `item` and print each as `item qty`.

## Run

    cargo run
