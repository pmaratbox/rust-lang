//! distinctUntilChanged built on a hand-rolled push-based Observable.
//!
//! An Observer is a closure that receives `next` values. `subscribe` wires a
//! producer to it. `distinct_until_changed` remembers the last forwarded value
//! and only pushes a new value when it differs from that last one.

/// A push-based Observable: a producer that, when subscribed, drives values
/// into the supplied observer closure.
struct Observable {
    subscribe: Box<dyn Fn(&mut dyn FnMut(i32))>,
}

impl Observable {
    /// Build an Observable from a fixed slice of values, emitted in order.
    fn from_values(values: Vec<i32>) -> Observable {
        Observable {
            subscribe: Box::new(move |observer: &mut dyn FnMut(i32)| {
                for v in &values {
                    observer(*v);
                }
            }),
        }
    }

    /// Forward a value only when it differs from the previously forwarded one.
    fn distinct_until_changed(self) -> Observable {
        Observable {
            subscribe: Box::new(move |observer: &mut dyn FnMut(i32)| {
                let mut last: Option<i32> = None;
                (self.subscribe)(&mut |v| {
                    if last != Some(v) {
                        last = Some(v);
                        observer(v);
                    }
                });
            }),
        }
    }

    /// Subscribe with a `next` handler.
    fn subscribe(&self, mut next: impl FnMut(i32)) {
        (self.subscribe)(&mut next);
    }
}

fn main() {
    let source = Observable::from_values(vec![1, 1, 2, 2, 2, 3, 1]);
    source
        .distinct_until_changed()
        .subscribe(|v| println!("{}", v));
}
