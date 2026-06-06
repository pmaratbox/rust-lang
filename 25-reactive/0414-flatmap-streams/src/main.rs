// FlatMap (mergeMap): map each outer value to an inner timed stream and merge
// all inners concurrently (no cancellation). A virtual-time scheduler drives
// every emission deterministically — no real threads, timers, or clocks.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

/// A single scheduled task: fire `callback` at virtual time `time`.
/// `seq` preserves insertion order so ties at equal time are stable.
struct Task {
    time: u64,
    seq: u64,
    callback: Box<dyn FnOnce(&Scheduler)>,
}

// BinaryHeap is a max-heap, so invert the ordering to pop the
// smallest (time, seq) first.
impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.seq == other.seq
    }
}
impl Eq for Task {}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .time
            .cmp(&self.time)
            .then_with(|| other.seq.cmp(&self.seq))
    }
}
impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Virtual-time scheduler: a priority queue of (time, seq, callback).
struct Scheduler {
    queue: RefCell<BinaryHeap<Task>>,
    clock: RefCell<u64>,
    next_seq: RefCell<u64>,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            queue: RefCell::new(BinaryHeap::new()),
            clock: RefCell::new(0),
            next_seq: RefCell::new(0),
        }
    }

    fn now(&self) -> u64 {
        *self.clock.borrow()
    }

    /// Enqueue `cb` to run at virtual time `at`.
    fn schedule(&self, at: u64, cb: Box<dyn FnOnce(&Scheduler)>) {
        let seq = {
            let mut s = self.next_seq.borrow_mut();
            let v = *s;
            *s += 1;
            v
        };
        self.queue.borrow_mut().push(Task {
            time: at,
            seq,
            callback: cb,
        });
    }

    /// Pop the smallest (time, seq) repeatedly, advance the clock, run it.
    fn run(&self) {
        loop {
            let task = match self.queue.borrow_mut().pop() {
                Some(t) => t,
                None => break,
            };
            *self.clock.borrow_mut() = task.time;
            (task.callback)(self);
        }
    }
}

/// Push-based observable. An observer is a closure receiving (scheduler, value);
/// `subscribe` wires a producer to that observer.
type Observer = Rc<dyn Fn(&Scheduler, i64)>;
struct Source {
    subscribe: Box<dyn Fn(&Scheduler, Observer)>,
}

/// Outer source: emits 1 at t=10 and 2 at t=20.
fn outer() -> Source {
    Source {
        subscribe: Box::new(|sched: &Scheduler, obs: Observer| {
            let o1 = obs.clone();
            sched.schedule(10, Box::new(move |s| o1(s, 1)));
            let o2 = obs.clone();
            sched.schedule(20, Box::new(move |s| o2(s, 2)));
        }),
    }
}

/// Inner source for value n: emits n at now+5 and n*10 at now+30.
fn inner(n: i64) -> Source {
    Source {
        subscribe: Box::new(move |sched: &Scheduler, obs: Observer| {
            let now = sched.now();
            let o1 = obs.clone();
            sched.schedule(now + 5, Box::new(move |s| o1(s, n)));
            let o2 = obs.clone();
            sched.schedule(now + 30, Box::new(move |s| o2(s, n * 10)));
        }),
    }
}

/// flatMap/mergeMap: each outer value spawns an inner stream; all inners run
/// concurrently and their emissions are merged downstream (no cancellation).
fn flat_map(source: Source, project: Rc<dyn Fn(i64) -> Source>) -> Source {
    Source {
        subscribe: Box::new(move |sched: &Scheduler, obs: Observer| {
            let project = project.clone();
            let downstream = obs.clone();
            // On each outer value, subscribe its inner against the live clock.
            let outer_obs: Observer = Rc::new(move |s: &Scheduler, n: i64| {
                let inner_src = project(n);
                (inner_src.subscribe)(s, downstream.clone());
            });
            (source.subscribe)(sched, outer_obs);
        }),
    }
}

fn main() {
    let sched = Scheduler::new();

    let project: Rc<dyn Fn(i64) -> Source> = Rc::new(inner);
    let merged = flat_map(outer(), project);

    let sink: Observer = Rc::new(|_s: &Scheduler, v: i64| println!("{}", v));

    (merged.subscribe)(&sched, sink);
    sched.run();
}
