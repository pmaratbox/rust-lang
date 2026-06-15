# 0509 — Nested object

Defines a GraphQL schema with `async-graphql` where `Query.user` returns a `User` whose `address` field is itself an `Address` object (declared with `#[derive(SimpleObject)]`). The query `{ user { address { city } } }` selects a field through **nested object types**; it is executed in-process (no HTTP server) and the resolved value is extracted from the result data and printed: `oslo`.

## Run

    cargo run
