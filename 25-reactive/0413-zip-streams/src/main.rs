use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

/// A minimal push-based observer: just a closure invoked with each value.
type Observer<T> = Rc<dyn Fn(T)>;

/// A push-based observable: subscribing wires a producer to an observer.
struct Observable<T> {
    subscribe: Box<dyn Fn(Observer<T>)>,
}

impl<T: 'static> Observable<T> {
    fn new(subscribe: impl Fn(Observer<T>) + 'static) -> Self {
        Observable {
            subscribe: Box::new(subscribe),
        }
    }
}

/// Emit a fixed sequence of values synchronously.
fn from_iter<T: Clone + 'static>(items: Vec<T>) -> Observable<T> {
    Observable::new(move |obs| {
        for item in &items {
            obs(item.clone());
        }
    })
}

/// zip: pair values by index and combine them. Each source is buffered in its
/// own queue; whenever both queues are non-empty we dequeue one from each and
/// emit combine(x, y).
fn zip(
    a: Observable<i32>,
    b: Observable<i32>,
    combine: impl Fn(i32, i32) -> i32 + 'static,
) -> Observable<i32> {
    let combine = Rc::new(combine);
    Observable::new(move |out| {
        let qa: Rc<RefCell<VecDeque<i32>>> = Rc::new(RefCell::new(VecDeque::new()));
        let qb: Rc<RefCell<VecDeque<i32>>> = Rc::new(RefCell::new(VecDeque::new()));

        // Drain helper: while both queues have a value, emit the combination.
        let drain = {
            let qa = Rc::clone(&qa);
            let qb = Rc::clone(&qb);
            let out = Rc::clone(&out);
            let combine = Rc::clone(&combine);
            move || {
                while !qa.borrow().is_empty() && !qb.borrow().is_empty() {
                    let x = qa.borrow_mut().pop_front().unwrap();
                    let y = qb.borrow_mut().pop_front().unwrap();
                    out((combine)(x, y));
                }
            }
        };
        let drain = Rc::new(drain);

        let on_a: Observer<i32> = {
            let qa = Rc::clone(&qa);
            let drain = Rc::clone(&drain);
            Rc::new(move |v| {
                qa.borrow_mut().push_back(v);
                drain();
            })
        };
        let on_b: Observer<i32> = {
            let qb = Rc::clone(&qb);
            let drain = Rc::clone(&drain);
            Rc::new(move |v| {
                qb.borrow_mut().push_back(v);
                drain();
            })
        };

        (a.subscribe)(on_a);
        (b.subscribe)(on_b);
    })
}

fn main() {
    let a = from_iter(vec![1, 2, 3]);
    let b = from_iter(vec![10, 20, 30]);
    let zipped = zip(a, b, |x, y| x + y);
    (zipped.subscribe)(Rc::new(|v| println!("{}", v)));
}
