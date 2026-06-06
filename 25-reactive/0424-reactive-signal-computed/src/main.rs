use std::cell::RefCell;
use std::rc::Rc;

// A writable signal: holds a value and a list of subscriber callbacks.
struct SignalInner {
    value: i64,
    subscribers: Vec<Rc<dyn Fn()>>,
}

#[derive(Clone)]
struct Signal {
    inner: Rc<RefCell<SignalInner>>,
}

impl Signal {
    fn new(value: i64) -> Self {
        Signal {
            inner: Rc::new(RefCell::new(SignalInner {
                value,
                subscribers: Vec::new(),
            })),
        }
    }

    fn get(&self) -> i64 {
        self.inner.borrow().value
    }

    fn subscribe(&self, cb: Rc<dyn Fn()>) {
        self.inner.borrow_mut().subscribers.push(cb);
    }

    fn set(&self, value: i64) {
        let subs = {
            let mut inner = self.inner.borrow_mut();
            inner.value = value;
            inner.subscribers.clone()
        };
        for cb in subs {
            cb();
        }
    }
}

// A computed: caches its result, recomputes when a dependency notifies.
#[derive(Clone)]
struct Computed {
    cache: Rc<RefCell<i64>>,
}

impl Computed {
    fn new<F>(deps: &[&Signal], compute: F) -> Self
    where
        F: Fn() -> i64 + 'static,
    {
        let cache = Rc::new(RefCell::new(compute()));
        let compute = Rc::new(compute);
        for dep in deps {
            let cache = cache.clone();
            let compute = compute.clone();
            dep.subscribe(Rc::new(move || {
                *cache.borrow_mut() = compute();
            }));
        }
        Computed { cache }
    }

    fn get(&self) -> i64 {
        *self.cache.borrow()
    }
}

fn main() {
    let a = Signal::new(2);
    let b = Signal::new(3);

    let sum = {
        let ca = a.clone();
        let cb = b.clone();
        Computed::new(&[&a, &b], move || ca.get() + cb.get())
    };

    println!("{}", sum.get());

    a.set(10);

    println!("{}", sum.get());
}
