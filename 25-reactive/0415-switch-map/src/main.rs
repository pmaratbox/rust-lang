use std::cell::{Cell, RefCell};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::rc::Rc;

// A scheduled task: fires at `time`, ties broken by insertion `seq`.
// `alive` is a shared cancellation token shared by all events of one inner.
struct Task {
    time: u64,
    seq: u64,
    alive: Rc<Cell<bool>>,
    cb: Box<dyn FnOnce()>,
}

// Min-heap by (time, seq): reverse the natural ordering.
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

struct Scheduler {
    queue: BinaryHeap<Task>,
    seq: u64,
    now: u64,
}

impl Scheduler {
    fn new() -> Self {
        Scheduler {
            queue: BinaryHeap::new(),
            seq: 0,
            now: 0,
        }
    }

    fn schedule(&mut self, time: u64, alive: Rc<Cell<bool>>, cb: Box<dyn FnOnce()>) {
        let seq = self.seq;
        self.seq += 1;
        self.queue.push(Task {
            time,
            seq,
            alive,
            cb,
        });
    }

    fn now(&self) -> u64 {
        self.now
    }

    fn run(sched: &Rc<RefCell<Scheduler>>) {
        loop {
            let task = {
                let mut s = sched.borrow_mut();
                match s.queue.pop() {
                    Some(t) => {
                        s.now = t.time;
                        t
                    }
                    None => break,
                }
            };
            if task.alive.get() {
                (task.cb)();
            }
        }
    }
}

fn main() {
    let sched = Rc::new(RefCell::new(Scheduler::new()));

    // sink for emitted inner values
    let out: Rc<RefCell<Vec<u64>>> = Rc::new(RefCell::new(Vec::new()));

    // token of the currently-active inner; flipping it false cancels its pending events
    let active: Rc<RefCell<Option<Rc<Cell<bool>>>>> = Rc::new(RefCell::new(None));

    // switchMap onto each outer value: cancel previous inner, start a new one.
    let switch_to = {
        let sched = Rc::clone(&sched);
        let out = Rc::clone(&out);
        let active = Rc::clone(&active);
        move |n: u64| {
            // cancel previous inner's still-pending emissions
            if let Some(prev) = active.borrow_mut().take() {
                prev.set(false);
            }
            let token = Rc::new(Cell::new(true));
            *active.borrow_mut() = Some(Rc::clone(&token));

            let now = sched.borrow().now();
            // inner(n): (now+5 -> n), (now+30 -> n*10)
            {
                let out = Rc::clone(&out);
                sched.borrow_mut().schedule(
                    now + 5,
                    Rc::clone(&token),
                    Box::new(move || out.borrow_mut().push(n)),
                );
            }
            {
                let out = Rc::clone(&out);
                sched.borrow_mut().schedule(
                    now + 30,
                    Rc::clone(&token),
                    Box::new(move || out.borrow_mut().push(n * 10)),
                );
            }
        }
    };

    // outer source: (10 -> 1), (20 -> 2). Outer never cancels, so use an always-alive token.
    let outer_alive = Rc::new(Cell::new(true));
    for &(t, v) in &[(10u64, 1u64), (20u64, 2u64)] {
        let switch_to = switch_to.clone();
        sched.borrow_mut().schedule(
            t,
            Rc::clone(&outer_alive),
            Box::new(move || switch_to(v)),
        );
    }

    Scheduler::run(&sched);

    for v in out.borrow().iter() {
        println!("{}", v);
    }
}
