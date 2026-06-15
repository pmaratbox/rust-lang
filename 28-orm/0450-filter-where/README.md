# 0450 — Filter with where

Define a `users` model (id, name, age) with Diesel's `table!` macro and `#[derive(Queryable, Insertable)]`, open an in-memory SQLite database via `SqliteConnection::establish(":memory:")`, and insert three rows through `diesel::insert_into`. The query uses Diesel's query DSL — `users::table.filter(users::age.ge(30)).order(users::id).load::<User>()` — to select users aged 30 or older, ordered by id, printing each name.

## Run

    cargo run
