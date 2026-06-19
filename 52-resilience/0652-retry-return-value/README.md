# 0652 — Return a value

Uses the `retry` crate to drive a scripted operation that fails once and then
returns the string `ok`. A shared `Cell` counter scripts the failure sequence;
once the closure produces `Ok`, the library stops retrying and hands back the
successful value, which we print (the returned value, not the attempt count).

## Run

    cargo run
