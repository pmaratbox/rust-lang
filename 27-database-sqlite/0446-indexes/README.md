# 0446 — Indexes

This lesson uses the real `rusqlite` driver (bundled SQLite) against an
in-memory database. It creates a `products` table, inserts three rows, then
executes `create index idx_sku on products(sku)` to build a secondary index.
A prepared statement `select price from products where sku=?` is bound to
`'B'` and run with `query_row`, printing the matching price.

## Run

    cargo run
