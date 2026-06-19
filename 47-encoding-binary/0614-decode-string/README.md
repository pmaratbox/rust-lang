# 0614 — Decode bytes

This lesson uses the `rmp-serde` MessagePack library to decode bytes back into a
value. We start from the hex string `a568656c6c6f`, convert it to a byte slice,
and hand it to `rmp_serde::from_slice::<String>` which reads the "fixstr" header
(`0xa5` for length 5) and the following UTF-8 bytes, yielding `hello`.

## Run

    cargo run
