# 0228 — Parse Quoted CSV

Parse the CSV row `a,"b,c",d`, respecting the quoted comma, into three fields joined by pipes `a|b,c|d`. Rust tracks an `in_quotes` flag and uses `std::mem::take` to flush each field.

## Run

    cargo run
