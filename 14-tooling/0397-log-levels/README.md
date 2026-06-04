# 0397 — Log Level Filter

With a threshold of WARN, log messages at INFO, WARN, and ERROR but only emit WARN and ERROR, on two lines. A `PartialOrd`-derived enum makes the level comparison a plain `>=`.

## Run

    cargo run
