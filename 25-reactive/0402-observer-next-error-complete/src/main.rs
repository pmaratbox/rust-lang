// Observer contract: next* then a single terminal (complete or error).
// A `stopped` flag is set on the first terminal; afterwards next() and
// further terminals are no-ops.

struct Observer {
    stopped: bool,
}

impl Observer {
    fn new() -> Self {
        Observer { stopped: false }
    }

    fn next(&mut self, value: i32) {
        if self.stopped {
            return;
        }
        println!("{}", value);
    }

    #[allow(dead_code)]
    fn error(&mut self, message: &str) {
        if self.stopped {
            return;
        }
        self.stopped = true;
        println!("error: {}", message);
    }

    fn complete(&mut self) {
        if self.stopped {
            return;
        }
        self.stopped = true;
        println!("complete");
    }
}

fn main() {
    let mut observer = Observer::new();
    observer.next(1);
    observer.next(2);
    observer.complete();
    observer.next(3); // ignored: observer already stopped
}
