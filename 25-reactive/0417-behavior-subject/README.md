# 0417 — BehaviorSubject

Implement a BehaviorSubject that holds a current value and replays it immediately to each new subscriber. Observers are stored as `Rc<dyn Fn(i64)>` closures so a single subject can fan out to many subscribers.

## Run

    cargo run
