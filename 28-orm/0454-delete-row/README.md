# 0454 — Delete a row

Use the `diesel` query builder over an in-memory SQLite database (bundled via `libsqlite3-sys`). After inserting three users with `diesel::insert_into`, remove the row with `id = 1` using `diesel::delete(users::table.filter(users::id.eq(1)))`, then load the remaining users ordered by `id` with the query DSL and print each name.

## Run

    cargo run
