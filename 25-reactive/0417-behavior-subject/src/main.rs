use std::cell::RefCell;
use std::rc::Rc;

/// An observer is a closure receiving the next value.
type Observer = Rc<dyn Fn(i64)>;

/// A BehaviorSubject holds a current value and replays it immediately to each
/// new subscriber, then forwards every subsequent `next`.
struct BehaviorSubject {
    current: i64,
    observers: Vec<Observer>,
}

impl BehaviorSubject {
    fn new(seed: i64) -> Self {
        BehaviorSubject {
            current: seed,
            observers: Vec::new(),
        }
    }

    /// Register an observer; it is immediately replayed the current value.
    fn subscribe(&mut self, observer: Observer) {
        observer(self.current);
        self.observers.push(observer);
    }

    /// Update the current value and push it to all observers.
    fn next(&mut self, value: i64) {
        self.current = value;
        for observer in &self.observers {
            observer(value);
        }
    }
}

fn main() {
    let subject = Rc::new(RefCell::new(BehaviorSubject::new(0)));

    let observer_a: Observer = Rc::new(|v| println!("A: {}", v));
    subject.borrow_mut().subscribe(observer_a);

    subject.borrow_mut().next(1);

    let observer_b: Observer = Rc::new(|v| println!("B: {}", v));
    subject.borrow_mut().subscribe(observer_b);

    subject.borrow_mut().next(2);
}
