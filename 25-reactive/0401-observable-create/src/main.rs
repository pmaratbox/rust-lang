// A push-based Observable built from scratch.
// An Observer receives `next` values and a `complete` signal.

struct Observer<'a> {
    next: Box<dyn FnMut(i32) + 'a>,
    complete: Box<dyn FnMut() + 'a>,
}

// An Observable is a function that, on subscribe, drives an observer.
struct Observable<F: Fn(&mut Observer)> {
    subscribe: F,
}

impl<F: Fn(&mut Observer)> Observable<F> {
    fn new(subscribe: F) -> Self {
        Observable { subscribe }
    }

    fn subscribe(&self, observer: &mut Observer) {
        (self.subscribe)(observer);
    }
}

fn main() {
    let source = Observable::new(|observer: &mut Observer| {
        (observer.next)(1);
        (observer.next)(2);
        (observer.next)(3);
        (observer.complete)();
    });

    let mut observer = Observer {
        next: Box::new(|value| println!("{}", value)),
        complete: Box::new(|| println!("done")),
    };

    source.subscribe(&mut observer);
}
