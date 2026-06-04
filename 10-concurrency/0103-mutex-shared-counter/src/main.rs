use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let counter = Arc::clone(&counter);
            thread::spawn(move || {
                for _ in 0..100 {
                    let mut n = counter.lock().unwrap();
                    *n += 1;
                }
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("{}", *counter.lock().unwrap());
}
