# 0106 — Producer / Consumer

A producer sends 1..5 to a consumer that sums them, printing `15`. A bounded `mpsc::sync_channel` applies backpressure while the consumer folds the received values.

## Run

    cargo run
