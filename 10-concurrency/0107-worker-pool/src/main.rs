use std::sync::{mpsc, Arc, Mutex};
use std::thread;

fn main() {
    let (job_tx, job_rx) = mpsc::channel::<i32>();
    let (res_tx, res_rx) = mpsc::channel::<i32>();
    let job_rx = Arc::new(Mutex::new(job_rx));

    // Pool of 2 workers, each squaring jobs pulled from the shared queue.
    let mut workers = Vec::new();
    for _ in 0..2 {
        let job_rx = Arc::clone(&job_rx);
        let res_tx = res_tx.clone();
        workers.push(thread::spawn(move || loop {
            let job = {
                let rx = job_rx.lock().unwrap();
                rx.recv()
            };
            match job {
                Ok(n) => res_tx.send(n * n).unwrap(),
                Err(_) => break,
            }
        }));
    }
    drop(res_tx);

    for n in 1..=4 {
        job_tx.send(n).unwrap();
    }
    drop(job_tx);

    let mut results: Vec<i32> = res_rx.iter().collect();
    for w in workers {
        w.join().unwrap();
    }

    results.sort();
    let line: Vec<String> = results.iter().map(|n| n.to_string()).collect();
    println!("{}", line.join(" "));
}
