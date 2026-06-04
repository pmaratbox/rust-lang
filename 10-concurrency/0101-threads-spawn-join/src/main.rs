use std::thread;

fn main() {
    let handles: Vec<_> = (0..3)
        .map(|_| thread::spawn(|| { /* trivial work */ }))
        .collect();

    let mut joined = 0;
    for h in handles {
        h.join().unwrap();
        joined += 1;
    }

    println!("done: {}", joined);
}
