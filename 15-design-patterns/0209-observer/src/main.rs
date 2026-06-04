trait Observer {
    fn update(&self, value: i32);
}

struct ConcreteObserver {
    id: String,
}
impl Observer for ConcreteObserver {
    fn update(&self, value: i32) {
        println!("{}: {}", self.id, value);
    }
}

struct Subject {
    observers: Vec<Box<dyn Observer>>,
}
impl Subject {
    fn new() -> Self {
        Subject { observers: Vec::new() }
    }
    fn register(&mut self, o: Box<dyn Observer>) {
        self.observers.push(o);
    }
    fn notify(&self, value: i32) {
        for o in &self.observers {
            o.update(value);
        }
    }
}

fn main() {
    let mut subject = Subject::new();
    subject.register(Box::new(ConcreteObserver { id: "obs1".to_string() }));
    subject.register(Box::new(ConcreteObserver { id: "obs2".to_string() }));
    subject.notify(5);
}
