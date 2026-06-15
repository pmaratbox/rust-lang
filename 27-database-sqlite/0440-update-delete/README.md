# 0440 — Update & delete

Open an in-memory SQLite database with the `rusqlite` driver (using the `bundled` SQLite feature), create a `users` table and insert three rows, then run an `update` to rename one row and a `delete` to remove another. It uses `Connection::execute` for the parameterized inserts and the mutating statements, then `prepare` plus `query_map` to read back the remaining rows ordered by `id` and print each as `id name`.

## Run

    cargo run
