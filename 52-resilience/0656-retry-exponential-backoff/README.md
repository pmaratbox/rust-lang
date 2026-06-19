# 0656 — Exponential backoff

Uses the `retry` crate with its `Exponential` delay strategy (zero base delay so
the run stays instant). A scripted closure tracked by a shared `Cell` counter
fails three times and succeeds on the fourth call. The crate keeps retrying
under the exponential schedule until the operation returns `Ok`, so the program
prints the total attempt count `4`.

## Run

    cargo run
