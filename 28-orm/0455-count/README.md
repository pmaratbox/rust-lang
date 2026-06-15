# 0455 — Count

Open an in-memory SQLite database with the `diesel` query builder, define the `users` table via the `diesel::table!` macro plus `Queryable`/`Insertable` derives, insert three rows, then run a count aggregate through the DSL with `users::table.select(diesel::dsl::count_star()).first()` and print the total row count.

## Run

    cargo run
