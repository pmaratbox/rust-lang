# 0045 — Abstract Classes & Methods

Define an abstract `Shape` with an abstract `area` and a concrete `describe` that uses it, then implement a `Square` of side 3 and print `area: 9`. Rust has no classes; a *trait* with a required method (`area`) and a *default* method (`describe`) is exactly an abstract base. `Square` implements `area` and inherits `describe`.

## Run

    cargo run
