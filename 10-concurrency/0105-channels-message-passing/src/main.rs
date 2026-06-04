use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        for v in [1, 2, 3] {
            tx.send(v).unwrap();
        }
        // tx dropped here, closing the channel
    });

    let received: Vec<String> = rx.iter().map(|v| v.to_string()).collect();
    println!("{}", received.join(" "));
}
