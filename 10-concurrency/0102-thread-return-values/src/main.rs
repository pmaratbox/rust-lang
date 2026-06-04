use std::thread;

fn main() {
    let t1 = thread::spawn(|| 3 * 3);
    let t2 = thread::spawn(|| 4 * 4);

    let a = t1.join().unwrap();
    let b = t2.join().unwrap();

    println!("{}", a + b);
}
