# 0508 — List field

Defines a GraphQL schema with `async-graphql` where `Query.numbers` is a list field (`[Int]`) whose resolver returns `[1, 2, 3]`, executes `{ numbers }` in-process, and prints each element of the resolved list on its own line.

## Run

    cargo run
