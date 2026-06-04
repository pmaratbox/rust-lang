use std::thread;

fn main() {
    let task_a = thread::spawn(|| 10);
    let task_b = thread::spawn(|| 20);

    let a = task_a.join().unwrap();
    let b = task_b.join().unwrap();

    println!("{}", a + b);
}
