# 0105 — Channels / Message Passing

Send the values 1, 2, 3 through a channel (or queue) from one thread and receive them in order, printing `1 2 3`. `mpsc::channel` gives a sender/receiver pair, and iterating the receiver yields values until the sender is dropped.

## Run

    cargo run
