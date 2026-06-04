use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Once};
use std::thread;

fn main() {
    let init = Arc::new(Once::new());
    let count = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..10)
        .map(|_| {
            let init = Arc::clone(&init);
            let count = Arc::clone(&count);
            thread::spawn(move || {
                init.call_once(|| {
                    count.fetch_add(1, Ordering::SeqCst);
                });
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("init count: {}", count.load(Ordering::SeqCst));
}
