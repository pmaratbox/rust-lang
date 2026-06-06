// A push-based Observable implemented from scratch.
//
// An Observer receives `next`/`complete`. `subscribe` wires a producer to the
// observer and returns a Subscription whose `unsubscribe` flips a shared
// "active" flag so the producer stops being driven. `take(n)` counts emissions
// and, after the n-th, calls `complete` and unsubscribes the source so the
// infinite source halts.

use std::cell::Cell;
use std::rc::Rc;

struct Subscription {
    active: Rc<Cell<bool>>,
}

impl Subscription {
    fn unsubscribe(&self) {
        self.active.set(false);
    }
}

struct Observer {
    next: Box<dyn FnMut(i64)>,
    complete: Box<dyn FnMut()>,
}

// Source: the unbounded natural numbers 1, 2, 3, ... `subscribe` drives the
// producer while the shared `active` flag stays true; unsubscribing flips it so
// the loop stops being driven.
fn naturals_subscribe(mut observer: Observer, active: Rc<Cell<bool>>) -> Subscription {
    let active_loop = active.clone();
    let mut n: i64 = 0;
    while active_loop.get() {
        n += 1;
        (observer.next)(n);
    }
    // Source exhausted/stopped: deliver the terminal completion notification.
    (observer.complete)();
    Subscription { active }
}

// take(n): pass through the first n emissions, then complete and unsubscribe the
// source. The downstream observer shares the `active` flag, so flipping it from
// inside `next` immediately stops the otherwise-infinite source.
fn take(n: usize) {
    let active = Rc::new(Cell::new(true));
    let count = Rc::new(Cell::new(0usize));

    let active_next = active.clone();
    let count_next = count.clone();

    let observer = Observer {
        next: Box::new(move |value: i64| {
            if count_next.get() >= n {
                return;
            }
            println!("{}", value);
            count_next.set(count_next.get() + 1);
            if count_next.get() >= n {
                // Reached the take limit: unsubscribe the source so it halts.
                active_next.set(false);
            }
        }),
        complete: Box::new(|| println!("completed")),
    };

    naturals_subscribe(observer, active);
}

fn main() {
    take(3);
}
