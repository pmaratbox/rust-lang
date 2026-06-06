# 0404 — Cold vs Hot Observable

Contrast a cold observable (re-runs its producer per subscriber) with a hot one (shares a single execution, so late subscribers miss earlier values). Rust models observers as `Rc<dyn Fn(i32)>` closures and shares the hot producer's subscriber list through a `RefCell`.

## Run

    cargo run
