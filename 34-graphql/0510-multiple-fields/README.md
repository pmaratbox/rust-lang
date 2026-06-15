# 0510 — Multiple fields

Defines a GraphQL schema with `async-graphql` where `Query.user` resolves to a `User` object (`#[derive(SimpleObject)]`) with `name` and `age` fields. The query `{ user { name age } }` **selects multiple fields** of that object and is executed in-process (no HTTP server). The resolved values are extracted from the result data and printed, each on its own line.

## Run

    cargo run
