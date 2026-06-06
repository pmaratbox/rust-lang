/// A push-based observable: subscribe wires a producer to an observer closure.
struct Observable<T> {
    subscribe: Box<dyn Fn(&mut dyn FnMut(T))>,
}

impl<T: 'static> Observable<T> {
    fn new(subscribe: impl Fn(&mut dyn FnMut(T)) + 'static) -> Self {
        Observable { subscribe: Box::new(subscribe) }
    }

    /// Forward a value only when the predicate holds.
    fn filter(self, pred: impl Fn(&T) -> bool + 'static) -> Observable<T> {
        Observable::new(move |observer| {
            (self.subscribe)(&mut |value| {
                if pred(&value) {
                    observer(value);
                }
            });
        })
    }
}

fn main() {
    // Source emits 1,2,3,4,5,6.
    let source = Observable::new(|observer: &mut dyn FnMut(i32)| {
        for v in 1..=6 {
            observer(v);
        }
    });

    let evens = source.filter(|v| v % 2 == 0);

    (evens.subscribe)(&mut |v| println!("{}", v));
}
