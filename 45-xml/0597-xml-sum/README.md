# 0597 — Sum numeric children

Parses the fixed catalog document with the `quick-xml` library (using its `serde` deserialization support) into `Catalog`/`Book` structs, then reads each `<book>`'s `<price>` child as an integer and sums them (30 + 45) to print `75`.

## Run

    cargo run
