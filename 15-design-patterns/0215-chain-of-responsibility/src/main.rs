trait Handler {
    fn handle(&self, level: i32);
}

struct ConcreteHandler {
    level: i32,
    next: Option<Box<dyn Handler>>,
}

impl Handler for ConcreteHandler {
    fn handle(&self, level: i32) {
        if self.level == level {
            println!("handled by {}", self.level);
        } else if let Some(next) = &self.next {
            next.handle(level);
        }
    }
}

fn main() {
    let chain = ConcreteHandler {
        level: 1,
        next: Some(Box::new(ConcreteHandler {
            level: 2,
            next: Some(Box::new(ConcreteHandler {
                level: 3,
                next: None,
            })),
        })),
    };
    chain.handle(2);
}
