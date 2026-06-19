# 0611 — Encode a nested array

This lesson uses the `rmp-serde` MessagePack library to encode the nested array
`[[1, 2], [3, 4]]` and print the lowercase hex of the resulting bytes. The outer
two-element fixarray (`92`) contains two inner two-element fixarrays (`92`), each
holding positive fixints, so the encoding is `92920102920304`.

## Run

    cargo run
