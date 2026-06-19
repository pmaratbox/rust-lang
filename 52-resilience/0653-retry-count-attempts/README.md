# 0653 — Count attempts

Uses the `retry` crate to drive a scripted operation that always fails. With
`Fixed::from_millis(0).take(4)` the library makes the first try plus up to four
retries — five attempts total — at zero delay. A shared `Cell` counter is bumped
each time the closure runs, so once the retries are exhausted it holds the total
number of attempts the library actually made. The program prints `5`.

## Run

    cargo run
