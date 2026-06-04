# 0205 — Decorator

Decorate a base coffee (cost 2) with milk (+1) and sugar (+1), printing the total cost `4`. Each decorator owns a `Box<dyn Coffee>` and adds to its `cost()`.

## Run

    cargo run
