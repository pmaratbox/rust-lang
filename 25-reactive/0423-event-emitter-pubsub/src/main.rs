use std::collections::HashMap;

type Handler = Box<dyn Fn(&str)>;

/// A multi-topic pub/sub EventEmitter. Each topic maps to a list of handlers,
/// each tagged with an id so `off` can remove a specific subscription.
struct EventEmitter {
    handlers: HashMap<String, Vec<(usize, Handler)>>,
}

impl EventEmitter {
    fn new() -> Self {
        EventEmitter {
            handlers: HashMap::new(),
        }
    }

    fn on(&mut self, topic: &str, id: usize, handler: Handler) {
        self.handlers
            .entry(topic.to_string())
            .or_default()
            .push((id, handler));
    }

    fn emit(&self, topic: &str, payload: &str) {
        if let Some(list) = self.handlers.get(topic) {
            for (_, handler) in list {
                handler(payload);
            }
        }
    }

    fn off(&mut self, topic: &str, id: usize) {
        if let Some(list) = self.handlers.get_mut(topic) {
            list.retain(|(hid, _)| *hid != id);
        }
    }
}

fn main() {
    let mut emitter = EventEmitter::new();

    let h_id = 1;
    let g_id = 2;

    emitter.on("greet", h_id, Box::new(|payload| println!("hi {}", payload)));
    emitter.on("bye", g_id, Box::new(|payload| println!("bye {}", payload)));

    emitter.emit("greet", "ada");
    emitter.emit("bye", "ada");

    emitter.off("greet", h_id);
    emitter.emit("greet", "x"); // handler removed: prints nothing
}
