use std::sync::{Arc, Barrier};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let n = 3;
    let barrier = Arc::new(Barrier::new(n));
    let reached = Arc::new(AtomicUsize::new(0));

    let handles: Vec<_> = (0..n)
        .map(|_| {
            let barrier = Arc::clone(&barrier);
            let reached = Arc::clone(&reached);
            thread::spawn(move || {
                reached.fetch_add(1, Ordering::SeqCst);
                barrier.wait();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }

    println!("all reached: {}", reached.load(Ordering::SeqCst));
}
