// A minimal push-based Observable implemented from scratch.
// An observer is a closure receiving each pushed value; subscribe wires a
// producer to it. scan keeps running state seeded with acc and emits f(state, v).

struct Observable<T> {
    subscribe: Box<dyn Fn(&mut dyn FnMut(T))>,
}

impl<T: 'static> Observable<T> {
    fn from_iter(values: Vec<T>) -> Observable<T>
    where
        T: Clone,
    {
        Observable {
            subscribe: Box::new(move |observer| {
                for v in values.iter().cloned() {
                    observer(v);
                }
            }),
        }
    }

    fn subscribe(&self, observer: &mut dyn FnMut(T)) {
        (self.subscribe)(observer);
    }
}

fn scan<T: 'static, A: 'static + Clone>(
    source: Observable<T>,
    acc: A,
    f: impl Fn(A, T) -> A + 'static,
) -> Observable<A> {
    Observable {
        subscribe: Box::new(move |observer| {
            let mut state = acc.clone();
            source.subscribe(&mut |value| {
                state = f(state.clone(), value);
                observer(state.clone());
            });
        }),
    }
}

fn main() {
    let source = Observable::from_iter(vec![1, 2, 3, 4]);
    let running = scan(source, 0, |state, value| state + value);
    running.subscribe(&mut |v| println!("{}", v));
}
