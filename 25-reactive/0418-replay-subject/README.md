# 0418 — ReplaySubject

Implement a ReplaySubject with a buffer of the last 2 values, replayed to a late subscriber, which then also receives new values. A `VecDeque` bounded to the capacity holds the replay buffer while `Rc<dyn Fn(T)>` observers are pushed on subscribe.

## Run

    cargo run
