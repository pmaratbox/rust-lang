# 0452 — Limit

Define a `users` model (id, name, age) with Diesel's `table!` macro and `#[derive(Queryable, Insertable)]`, open an in-memory SQLite database via `SqliteConnection::establish(":memory:")`, and insert three rows through `diesel::insert_into`. The query uses Diesel's query DSL — `users::table.order(users::age.desc()).limit(2).load::<User>()` — to take only the top two rows ordered by age descending, printing each name.

## Run

    cargo run
