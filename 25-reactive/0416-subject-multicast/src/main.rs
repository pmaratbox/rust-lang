// A Subject multicasts each emission to all current observers.
// It keeps a list of observers; subscribe appends; next(v) iterates
// observers calling each in registration order.

type Observer = Box<dyn Fn(i32)>;

struct Subject {
    observers: Vec<Observer>,
}

impl Subject {
    fn new() -> Self {
        Subject { observers: Vec::new() }
    }

    fn subscribe(&mut self, obs: Observer) {
        self.observers.push(obs);
    }

    fn next(&self, value: i32) {
        for obs in &self.observers {
            obs(value);
        }
    }
}

fn main() {
    let mut subject = Subject::new();

    subject.subscribe(Box::new(|v| println!("obs1: {}", v)));
    subject.subscribe(Box::new(|v| println!("obs2: {}", v)));

    subject.next(1);
    subject.next(2);
}
