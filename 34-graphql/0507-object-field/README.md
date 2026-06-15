# 0507 — Object field

Defines a GraphQL schema with `async-graphql` where a `#[derive(SimpleObject)]` `User` **object type** exposes a `name` field, and `Query.user` resolves to a `User`. The query `{ user { name } }` is executed in-process (no HTTP server) and the nested field is extracted from the result data and printed: `alice`.

## Run

    cargo run
