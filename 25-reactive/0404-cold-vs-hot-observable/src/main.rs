use std::cell::RefCell;
use std::rc::Rc;

// An observer is just a closure receiving the next value.
type Observer = Rc<dyn Fn(i32)>;

// A cold observable: subscribing runs the producer independently per subscriber.
struct Cold;

impl Cold {
    fn subscribe(&self, observer: Observer) {
        // The producer re-runs from scratch for every subscriber.
        for v in [1, 2, 3] {
            observer(v);
        }
    }
}

// A hot observable: a single shared producer multicasts to current subscribers.
struct Hot {
    observers: RefCell<Vec<Observer>>,
}

impl Hot {
    fn new() -> Self {
        Hot { observers: RefCell::new(Vec::new()) }
    }

    fn subscribe(&self, observer: Observer) {
        self.observers.borrow_mut().push(observer);
    }

    // The producer emits once to all currently subscribed observers.
    fn emit(&self, v: i32) {
        for obs in self.observers.borrow().iter() {
            obs(v);
        }
    }
}

fn collector() -> (Observer, Rc<RefCell<Vec<i32>>>) {
    let received = Rc::new(RefCell::new(Vec::new()));
    let sink = received.clone();
    let obs: Observer = Rc::new(move |v| sink.borrow_mut().push(v));
    (obs, received)
}

fn joined(values: &[i32]) -> String {
    values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(" ")
}

fn main() {
    // COLD: each subscriber triggers its own run of the producer.
    let cold = Cold;
    let (a_obs, a_vals) = collector();
    cold.subscribe(a_obs);
    let (b_obs, b_vals) = collector();
    cold.subscribe(b_obs);
    println!("cold A: {}", joined(&a_vals.borrow()));
    println!("cold B: {}", joined(&b_vals.borrow()));

    // HOT: one shared execution; late subscribers miss earlier values.
    let hot = Hot::new();
    let (ha_obs, ha_vals) = collector();
    hot.subscribe(ha_obs); // A subscribes first
    hot.emit(1); // only A is listening
    let (hb_obs, hb_vals) = collector();
    hot.subscribe(hb_obs); // B subscribes late
    hot.emit(2); // both receive
    hot.emit(3); // both receive
    println!("hot A: {}", joined(&ha_vals.borrow()));
    println!("hot B: {}", joined(&hb_vals.borrow()));
}
