use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::sync_channel(2); // bounded queue

    let producer = thread::spawn(move || {
        for v in 1..=5 {
            tx.send(v).unwrap();
        }
    });

    let sum: i32 = rx.iter().sum();
    producer.join().unwrap();

    println!("{}", sum);
}
