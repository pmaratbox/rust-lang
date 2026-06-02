# 0038 — String Methods

Split `"a,b,c"` on commas, upper-case each part, and join them with `-`, printing `A-B-C`. `split(',')` returns a lazy iterator, `map(str::to_uppercase)` upper-cases each piece, `collect` gathers a `Vec<String>`, and `join("-")` joins them. `to_uppercase` handles full Unicode, unlike a byte-wise upcase.

## Run

    cargo run
