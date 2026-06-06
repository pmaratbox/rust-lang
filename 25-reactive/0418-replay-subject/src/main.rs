use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// A push-based ReplaySubject implemented from scratch. It keeps a buffer of the
/// last `capacity` emitted values; a late subscriber immediately receives the
/// buffered values and then any subsequent emissions.
struct ReplaySubject<T: Clone> {
    capacity: usize,
    buffer: VecDeque<T>,
    observers: Vec<Rc<dyn Fn(T)>>,
}

impl<T: Clone + 'static> ReplaySubject<T> {
    fn new(capacity: usize) -> Self {
        ReplaySubject {
            capacity,
            buffer: VecDeque::new(),
            observers: Vec::new(),
        }
    }

    fn next(&mut self, value: T) {
        if self.buffer.len() == self.capacity {
            self.buffer.pop_front();
        }
        self.buffer.push_back(value.clone());
        for obs in &self.observers {
            obs(value.clone());
        }
    }

    fn subscribe(&mut self, observer: Rc<dyn Fn(T)>) {
        // Replay the buffered values to the late subscriber first.
        for v in &self.buffer {
            observer(v.clone());
        }
        self.observers.push(observer);
    }
}

fn main() {
    let subject: Rc<RefCell<ReplaySubject<i32>>> = Rc::new(RefCell::new(ReplaySubject::new(2)));

    // Emit 1,2,3 before any subscriber -> buffer keeps [2,3].
    subject.borrow_mut().next(1);
    subject.borrow_mut().next(2);
    subject.borrow_mut().next(3);

    // A late subscriber: receives buffered 2 then 3, then new values.
    let observer: Rc<dyn Fn(i32)> = Rc::new(|value| println!("{value}"));
    subject.borrow_mut().subscribe(observer);

    // Emit 4 -> subscriber receives 4.
    subject.borrow_mut().next(4);
}
