// A tiny push-based Observable implemented from scratch.
//
// An Observer is a trio of closures: next / error / complete. An Observable is
// just a subscribe function that wires a producer to an observer. retry(n)
// resubscribes to the source on error, up to n times.

struct Observer<'a> {
    next: &'a mut dyn FnMut(i32),
    error: &'a mut dyn FnMut(),
    complete: &'a mut dyn FnMut(),
}

// An Observable is a boxed subscribe function.
type Observable<'a> = Box<dyn Fn(&mut Observer) + 'a>;

// The source: each subscription increments a shared counter and prints the
// attempt. For k < 3 it errors; for k == 3 it emits then completes.
fn make_source(count: std::cell::RefCell<i32>) -> Observable<'static>
where
{
    Box::new(move |obs: &mut Observer| {
        let k = {
            let mut c = count.borrow_mut();
            *c += 1;
            *c
        };
        println!("attempt {}", k);
        if k < 3 {
            (obs.error)();
        } else {
            (obs.next)(42);
            (obs.complete)();
        }
    })
}

// retry(n): on error, resubscribe to the source up to n more times.
fn retry(source: &Observable, n: i32, obs: &mut Observer) {
    let mut remaining = n;
    let mut done = false;

    while !done {
        let mut errored = false;
        {
            let mut next = |v: i32| (obs.next)(v);
            let mut on_error = || {
                errored = true;
            };
            let mut complete = || (obs.complete)();
            let mut inner = Observer {
                next: &mut next,
                error: &mut on_error,
                complete: &mut complete,
            };
            source(&mut inner);
        }

        if errored {
            if remaining > 0 {
                remaining -= 1;
            } else {
                (obs.error)();
                done = true;
            }
        } else {
            done = true;
        }
    }
}

fn main() {
    let source = make_source(std::cell::RefCell::new(0));

    let mut next = |_v: i32| println!("ok");
    let mut error = || println!("error");
    let mut complete = || {};
    let mut obs = Observer {
        next: &mut next,
        error: &mut error,
        complete: &mut complete,
    };

    retry(&source, 2, &mut obs);
}
