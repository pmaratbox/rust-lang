//! Push-based Observable from scratch; concat subscribes to the second
//! source only after the first completes.

use std::cell::RefCell;
use std::rc::Rc;

/// Observer: closures for next and complete (error omitted for brevity).
struct Observer<'a> {
    next: Box<dyn FnMut(i32) + 'a>,
    complete: Box<dyn FnMut() + 'a>,
}

/// An Observable wraps a subscribe function that wires a producer to an observer.
struct Observable<'a> {
    subscribe: Box<dyn Fn(&mut Observer) + 'a>,
}

impl<'a> Observable<'a> {
    fn new(subscribe: impl Fn(&mut Observer) + 'a) -> Self {
        Observable { subscribe: Box::new(subscribe) }
    }

    /// Synchronous source that emits the given values then completes.
    fn of(values: Vec<i32>) -> Self {
        Observable::new(move |obs: &mut Observer| {
            for &v in &values {
                (obs.next)(v);
            }
            (obs.complete)();
        })
    }
}

/// concat(a, b): subscribe to a; on a.complete, subscribe to b; on b.complete,
/// complete. Synchronous, no scheduler needed.
fn concat<'a>(a: Observable<'a>, b: Observable<'a>) -> Observable<'a> {
    Observable::new(move |obs: &mut Observer| {
        // Share the downstream observer between both inner subscriptions.
        let shared: Rc<RefCell<&mut Observer>> = Rc::new(RefCell::new(obs));
        let b = &b;

        let next_a = shared.clone();
        let complete_shared = shared.clone();
        let mut a_obs = Observer {
            next: Box::new(move |v| (next_a.borrow_mut().next)(v)),
            complete: Box::new(move || {
                let next_b = complete_shared.clone();
                let complete_b = complete_shared.clone();
                let mut b_obs = Observer {
                    next: Box::new(move |v| (next_b.borrow_mut().next)(v)),
                    complete: Box::new(move || (complete_b.borrow_mut().complete)()),
                };
                (b.subscribe)(&mut b_obs);
            }),
        };
        (a.subscribe)(&mut a_obs);
    })
}

fn main() {
    let a = Observable::of(vec![1, 2]);
    let b = Observable::of(vec![3, 4]);
    let stream = concat(a, b);

    let mut observer = Observer {
        next: Box::new(|v| println!("{}", v)),
        complete: Box::new(|| {}),
    };
    (stream.subscribe)(&mut observer);
}
