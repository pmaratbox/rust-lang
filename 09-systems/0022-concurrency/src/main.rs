use std::thread;

fn main() {
    let a = thread::spawn(|| 1);
    let b = thread::spawn(|| 2);
    let sum = a.join().unwrap() + b.join().unwrap();
    println!("sum: {sum}");
}
