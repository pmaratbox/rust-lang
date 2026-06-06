// A push-based Observable implemented from scratch.
// An Observer carries next/error/complete callbacks; subscribe wires a
// producer to it. catchError forwards next values, but on error it
// subscribes to a fallback stream instead of propagating the error.

use std::cell::RefCell;
use std::rc::Rc;

struct Observer {
    next: Rc<RefCell<dyn FnMut(i32)>>,
    error: Rc<RefCell<dyn FnMut()>>,
    complete: Rc<RefCell<dyn FnMut()>>,
}

// An Observable is a producer: given an Observer, it emits events.
type Observable = Rc<dyn Fn(&Observer)>;

fn source() -> Observable {
    Rc::new(|obs: &Observer| {
        (obs.next.borrow_mut())(1);
        (obs.next.borrow_mut())(2);
        (obs.error.borrow_mut())();
    })
}

fn fallback() -> Observable {
    Rc::new(|obs: &Observer| {
        (obs.next.borrow_mut())(9);
        (obs.complete.borrow_mut())();
    })
}

fn catch_error(src: Observable, fb: Observable) -> Observable {
    Rc::new(move |obs: &Observer| {
        let next = obs.next.clone();
        let complete = obs.complete.clone();
        let fb = fb.clone();

        // Forward next as-is. On error, subscribe to the fallback with the
        // downstream observer instead of propagating the error.
        let inner_next = next.clone();
        let inner = Observer {
            next: Rc::new(RefCell::new(move |v| (inner_next.borrow_mut())(v))),
            error: Rc::new(RefCell::new({
                let next = next.clone();
                let error = obs.error.clone();
                let complete = complete.clone();
                move || {
                    let fb_obs = Observer {
                        next: next.clone(),
                        error: error.clone(),
                        complete: complete.clone(),
                    };
                    fb(&fb_obs);
                }
            })),
            complete: Rc::new(RefCell::new(move || (complete.borrow_mut())())),
        };
        src(&inner);
    })
}

fn main() {
    let stream = catch_error(source(), fallback());
    let obs = Observer {
        next: Rc::new(RefCell::new(|v| println!("{}", v))),
        error: Rc::new(RefCell::new(|| {})),
        complete: Rc::new(RefCell::new(|| {})),
    };
    stream(&obs);
}
