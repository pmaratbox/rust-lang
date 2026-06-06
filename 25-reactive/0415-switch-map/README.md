# 0415 — SwitchMap

Implement switchMap: when a new outer value arrives, cancel the previous inner subscription before starting the new one. Cancellation is modeled by flipping a shared `Rc<Cell<bool>>` token that the virtual scheduler checks before firing.

## Run

    cargo run
