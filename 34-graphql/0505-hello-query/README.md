# 0505 — Hello query

Builds a GraphQL schema with `async-graphql` whose `Query` type exposes a single `hello` field resolving to `"world"`, then executes the query `{ hello }` in-process (no HTTP server) and prints the resolved `data.hello` value extracted from the execution result.

## Run

    cargo run
