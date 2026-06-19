# 0651 — Retries exhausted

Uses the `retry` crate to drive a scripted operation that *always* fails. The
strategy `Fixed::from_millis(0).take(2)` permits up to two retries after the
first call, for 3 total attempts with zero delay. Because every attempt returns
`Err`, the crate gives up once the iterator is exhausted and returns an
`Err(RetryError)`. We catch that and print `failed`.

## Run

    cargo run
