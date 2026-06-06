use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

// A scheduled task in virtual time.
struct Task {
    time: u64,
    seq: u64,
    cancelled: bool,
    callback: Box<dyn FnMut(&Scheduler)>,
}

// Min-heap entry keyed by (time, seq); BinaryHeap is a max-heap so we invert Ord.
struct Entry {
    time: u64,
    seq: u64,
    index: usize,
}

impl PartialEq for Entry {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time && self.seq == other.seq
    }
}
impl Eq for Entry {}
impl Ord for Entry {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap: smallest (time, seq) pops first.
        other
            .time
            .cmp(&self.time)
            .then_with(|| other.seq.cmp(&self.seq))
    }
}
impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Scheduler {
    now: RefCell<u64>,
    seq: RefCell<u64>,
    tasks: RefCell<Vec<Option<Task>>>,
    heap: RefCell<BinaryHeap<Entry>>,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            now: RefCell::new(0),
            seq: RefCell::new(0),
            tasks: RefCell::new(Vec::new()),
            heap: RefCell::new(BinaryHeap::new()),
        }
    }

    fn now(&self) -> u64 {
        *self.now.borrow()
    }

    fn schedule(&self, time: u64, callback: Box<dyn FnMut(&Scheduler)>) -> usize {
        let seq = *self.seq.borrow();
        *self.seq.borrow_mut() += 1;
        let index = self.tasks.borrow().len();
        self.tasks.borrow_mut().push(Some(Task {
            time,
            seq,
            cancelled: false,
            callback,
        }));
        self.heap.borrow_mut().push(Entry { time, seq, index });
        index
    }

    #[allow(dead_code)]
    fn cancel(&self, token: usize) {
        if let Some(slot) = self.tasks.borrow_mut().get_mut(token) {
            if let Some(task) = slot {
                task.cancelled = true;
            }
        }
    }

    fn run(&self) {
        loop {
            let entry = match self.heap.borrow_mut().pop() {
                Some(e) => e,
                None => break,
            };
            // Take the callback out, skipping cancelled tasks.
            let mut task = {
                let mut tasks = self.tasks.borrow_mut();
                match tasks[entry.index].take() {
                    Some(t) => t,
                    None => continue,
                }
            };
            if task.cancelled {
                continue;
            }
            *self.now.borrow_mut() = task.time;
            (task.callback)(self);
        }
    }
}

// throttle (leading edge): emit a value only if now >= block_until.
fn main() {
    let scheduler = Scheduler::new();
    let block_until = Rc::new(RefCell::new(0u64));
    let window = 30u64;

    let source = [("a", 10u64), ("b", 20), ("c", 100), ("d", 110)];

    for (value, time) in source {
        let block_until = Rc::clone(&block_until);
        scheduler.schedule(
            time,
            Box::new(move |sched: &Scheduler| {
                let now = sched.now();
                if now >= *block_until.borrow() {
                    println!("{}", value);
                    *block_until.borrow_mut() = now + window;
                }
            }),
        );
    }

    scheduler.run();
}
