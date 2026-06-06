use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

/// A scheduled task in virtual time: (time, insertion-seq, callback).
struct Task {
    time: u64,
    seq: u64,
    dead: Rc<RefCell<bool>>,
    cb: Box<dyn FnMut(&Scheduler)>,
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.seq == other.seq
    }
}
impl Eq for Task {}
impl Ord for Task {
    // BinaryHeap is a max-heap; invert so the smallest (time, seq) pops first.
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

/// Virtual-time scheduler: a priority queue of timed callbacks.
struct Scheduler {
    heap: RefCell<BinaryHeap<Task>>,
    seq: RefCell<u64>,
    clock: RefCell<u64>,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            heap: RefCell::new(BinaryHeap::new()),
            seq: RefCell::new(0),
            clock: RefCell::new(0),
        }
    }

    fn schedule(&self, time: u64, cb: impl FnMut(&Scheduler) + 'static) -> Rc<RefCell<bool>> {
        let mut seq = self.seq.borrow_mut();
        let s = *seq;
        *seq += 1;
        let dead = Rc::new(RefCell::new(false));
        self.heap.borrow_mut().push(Task {
            time,
            seq: s,
            dead: dead.clone(),
            cb: Box::new(cb),
        });
        dead
    }

    fn run(&self) {
        loop {
            let task = self.heap.borrow_mut().pop();
            match task {
                None => break,
                Some(mut t) => {
                    if *t.dead.borrow() {
                        continue;
                    }
                    *self.clock.borrow_mut() = t.time;
                    (t.cb)(self);
                }
            }
        }
    }
}

fn main() {
    let scheduler = Scheduler::new();

    // Latest value of each source (None until first emission).
    let a_latest: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(None));
    let b_latest: Rc<RefCell<Option<i64>>> = Rc::new(RefCell::new(None));

    // On any emission, if both have a latest, emit the pair.
    let emit = {
        let a_latest = a_latest.clone();
        let b_latest = b_latest.clone();
        move || {
            if let (Some(a), Some(b)) = (*a_latest.borrow(), *b_latest.borrow()) {
                println!("({}, {})", a, b);
            }
        }
    };
    let emit = Rc::new(emit);

    // Source A schedules (1->1), (3->2).
    for &(t, v) in &[(1u64, 1i64), (3, 2)] {
        let a_latest = a_latest.clone();
        let emit = emit.clone();
        scheduler.schedule(t, move |_| {
            *a_latest.borrow_mut() = Some(v);
            emit();
        });
    }

    // Source B schedules (2->10).
    for &(t, v) in &[(2u64, 10i64)] {
        let b_latest = b_latest.clone();
        let emit = emit.clone();
        scheduler.schedule(t, move |_| {
            *b_latest.borrow_mut() = Some(v);
            emit();
        });
    }

    scheduler.run();
}
