# 0437 — Connect & query

Open an in-memory SQLite database with the `rusqlite` driver (using the `bundled` SQLite feature), run the single query `select 42`, and print the resulting integer. It uses `Connection::open_in_memory` to create the database and `query_row` to fetch the one-row, one-column result.

## Run

    cargo run
