# 0443 — Order by & limit

Open an in-memory SQLite database with the `rusqlite` driver (using the `bundled` SQLite feature), create a `scores` table and insert six integers, then run `select value from scores order by value desc limit 3` to sort descending and take the top three rows. It uses `Connection::open_in_memory`, a prepared statement, and `query_map` to iterate the rows, printing each value on its own line.

## Run

    cargo run
