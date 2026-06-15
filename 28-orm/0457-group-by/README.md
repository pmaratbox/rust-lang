# 0457 — Group by

Define a `products` model (id, category, price) with Diesel's `table!` macro and `#[derive(Insertable)]`, open an in-memory SQLite database via `SqliteConnection::establish(":memory:")`, and insert three rows through `diesel::insert_into`. The query uses Diesel's query DSL grouping API — `products::table.group_by(products::category).select((products::category, diesel::dsl::sum(products::price)))` — ordered by `category`, printing each `category sum` pair.

## Run

    cargo run
