use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;
use std::cell::RefCell;

/// A scheduled task: fires at `time`, ordered by (time, seq); `seq` breaks ties
/// by insertion order. `dead` marks a cancelled task to be skipped on pop.
struct Task {
    time: u64,
    seq: u64,
    dead: bool,
    cb: Box<dyn FnMut()>,
}

/// Reverse ordering so the BinaryHeap (a max-heap) behaves as a min-heap on
/// (time, seq): the smallest virtual time pops first, ties by insertion order.
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

/// Virtual-time scheduler: a priority queue of timed callbacks driven by run().
struct Scheduler {
    heap: BinaryHeap<Task>,
    seq: u64,
    clock: u64,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            heap: BinaryHeap::new(),
            seq: 0,
            clock: 0,
        }
    }

    fn schedule(&mut self, time: u64, cb: Box<dyn FnMut()>) {
        let seq = self.seq;
        self.seq += 1;
        self.heap.push(Task {
            time,
            seq,
            dead: false,
            cb,
        });
    }

    fn run(&mut self) {
        while let Some(mut task) = self.heap.pop() {
            if task.dead {
                continue;
            }
            self.clock = task.time;
            (task.cb)();
        }
    }
}

/// A push-based observable: subscribing schedules its emissions onto `sched`.
type Observer = Rc<RefCell<dyn FnMut(i32)>>;

struct Stream {
    events: Vec<(u64, i32)>,
}

impl Stream {
    fn subscribe(&self, sched: &mut Scheduler, observer: Observer) {
        for &(t, v) in &self.events {
            let obs = Rc::clone(&observer);
            sched.schedule(
                t,
                Box::new(move || {
                    (obs.borrow_mut())(v);
                }),
            );
        }
    }
}

/// merge: subscribe both streams onto the same observer; the scheduler
/// interleaves their emissions by virtual time.
fn merge(sched: &mut Scheduler, a: &Stream, b: &Stream, observer: Observer) {
    a.subscribe(sched, Rc::clone(&observer));
    b.subscribe(sched, observer);
}

fn main() {
    let mut sched = Scheduler::new();

    let a = Stream {
        events: vec![(10, 1), (30, 3), (50, 5)],
    };
    let b = Stream {
        events: vec![(20, 2), (40, 4), (60, 6)],
    };

    let observer: Observer = Rc::new(RefCell::new(|v: i32| println!("{}", v)));

    merge(&mut sched, &a, &b, observer);
    sched.run();
}
