use std::cell::Cell;
use std::rc::Rc;

/// A handle returned by `subscribe`; calling `unsubscribe` flips a shared
/// `closed` flag so the producer stops delivering further values.
#[derive(Clone)]
struct Subscription {
    closed: Rc<Cell<bool>>,
}

impl Subscription {
    fn unsubscribe(&self) {
        self.closed.set(true);
    }
}

/// A push-based observable: `subscribe` wires the producer to an observer
/// and returns a `Subscription` that can cancel delivery.
struct Observable<F> {
    producer: F,
}

impl<F> Observable<F>
where
    F: Fn(&dyn Fn(i32), &Subscription),
{
    fn new(producer: F) -> Self {
        Observable { producer }
    }

    fn subscribe(&self, observe: impl Fn(i32, &Subscription)) -> Subscription {
        let sub = Subscription {
            closed: Rc::new(Cell::new(false)),
        };
        let next = |value: i32| observe(value, &sub);
        (self.producer)(&next, &sub);
        sub
    }
}

fn main() {
    // The source would push 1,2,3,4 but checks `closed` before each `next`.
    let source = Observable::new(|next: &dyn Fn(i32), sub: &Subscription| {
        for value in 1..=4 {
            if sub.closed.get() {
                break;
            }
            next(value);
        }
    });

    // The consumer unsubscribes after receiving 2 so 3 and 4 never arrive.
    source.subscribe(|value, sub| {
        println!("{}", value);
        if value == 2 {
            sub.unsubscribe();
        }
    });
}
