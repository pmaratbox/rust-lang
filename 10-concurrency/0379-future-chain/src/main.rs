use std::thread;

fn main() {
    let result = thread::spawn(|| {
        let a = 5;
        let b = thread::spawn(move || a * 2).join().unwrap();
        thread::spawn(move || b + 1).join().unwrap()
    })
    .join()
    .unwrap();

    println!("{}", result);
}
