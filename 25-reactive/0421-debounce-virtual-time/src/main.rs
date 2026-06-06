use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

// A scheduled task on the virtual-time scheduler.
struct Task {
    time: u64,
    seq: u64,
    cancelled: Rc<RefCell<bool>>,
    cb: Box<dyn FnMut(&Scheduler)>,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.seq == other.seq
    }
}
impl Eq for Task {}
impl Ord for Task {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse so BinaryHeap (max-heap) pops the smallest (time, seq).
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

struct Scheduler {
    inner: RefCell<Inner>,
}

struct Inner {
    heap: BinaryHeap<Task>,
    seq: u64,
    now: u64,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            inner: RefCell::new(Inner {
                heap: BinaryHeap::new(),
                seq: 0,
                now: 0,
            }),
        }
    }

    fn now(&self) -> u64 {
        self.inner.borrow().now
    }

    // Schedule cb at virtual time `time`; returns a cancel token.
    fn schedule<F: FnMut(&Scheduler) + 'static>(&self, time: u64, cb: F) -> Rc<RefCell<bool>> {
        let mut inner = self.inner.borrow_mut();
        let seq = inner.seq;
        inner.seq += 1;
        let cancelled = Rc::new(RefCell::new(false));
        inner.heap.push(Task {
            time,
            seq,
            cancelled: cancelled.clone(),
            cb: Box::new(cb),
        });
        cancelled
    }

    fn run(&self) {
        loop {
            let task = {
                let mut inner = self.inner.borrow_mut();
                match inner.heap.pop() {
                    Some(t) => {
                        inner.now = t.time;
                        t
                    }
                    None => break,
                }
            };
            if *task.cancelled.borrow() {
                continue;
            }
            let mut cb = task.cb;
            cb(self);
        }
    }
}

fn cancel(token: &Rc<RefCell<bool>>) {
    *token.borrow_mut() = true;
}

fn main() {
    let scheduler = Rc::new(Scheduler::new());

    // Source events: ("a"@10),("b"@20),("c"@100).
    let events = [("a", 10u64), ("b", 20), ("c", 100)];
    let window = 30u64;

    // Pending emit token, shared across the closures.
    let pending: Rc<RefCell<Option<Rc<RefCell<bool>>>>> = Rc::new(RefCell::new(None));

    for (value, t) in events {
        let sched = scheduler.clone();
        let pending = pending.clone();
        scheduler.schedule(t, move |s| {
            // On each value, cancel any pending emit...
            if let Some(tok) = pending.borrow_mut().take() {
                cancel(&tok);
            }
            // ...and schedule this value to fire at now + window.
            let fire_at = s.now() + window;
            let value = value.to_string();
            let token = sched.schedule(fire_at, move |_| {
                println!("{}", value);
            });
            *pending.borrow_mut() = Some(token);
        });
    }

    scheduler.run();
}
