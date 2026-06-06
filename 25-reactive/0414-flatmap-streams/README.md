# 0414 — FlatMap (mergeMap)

Implement flatMap/mergeMap: map each outer value to an inner timed stream and merge all inners concurrently (no cancellation). In Rust the observer is an `Rc<dyn Fn(&Scheduler, i64)>` closure and tasks share the scheduler through `RefCell` interior mutability.

## Run

    cargo run
