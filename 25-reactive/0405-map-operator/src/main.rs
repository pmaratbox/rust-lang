// A tiny push-based Observable implemented from scratch.
// An Observer is just a closure receiving each emitted value.
// `subscribe` wires a producer to an observer; `map` returns a new
// Observable whose `next` forwards `f(value)` to the downstream observer.

struct Observable<T> {
    // The producer: given an observer (next callback), emit values into it.
    subscribe: Box<dyn Fn(&mut dyn FnMut(T))>,
}

impl<T: 'static> Observable<T> {
    fn new(subscribe: impl Fn(&mut dyn FnMut(T)) + 'static) -> Self {
        Observable {
            subscribe: Box::new(subscribe),
        }
    }

    fn subscribe_fn(&self, mut next: impl FnMut(T)) {
        (self.subscribe)(&mut next);
    }
}

// map(source, f) returns a new Observable whose next forwards f(value).
fn map<T: 'static, U: 'static>(
    source: Observable<T>,
    f: impl Fn(T) -> U + 'static,
) -> Observable<U> {
    Observable::new(move |downstream: &mut dyn FnMut(U)| {
        source.subscribe_fn(|value| downstream(f(value)));
    })
}

fn main() {
    // Source emits 1, 2, 3, 4.
    let source = Observable::new(|next: &mut dyn FnMut(i32)| {
        for v in [1, 2, 3, 4] {
            next(v);
        }
    });

    // f = *2; print 2, 4, 6, 8 one per line.
    let doubled = map(source, |x| x * 2);

    doubled.subscribe_fn(|value| println!("{}", value));
}
