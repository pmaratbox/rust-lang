use std::thread;

fn main() {
    let input = [1, 2, 3, 4];

    let handles: Vec<_> = input
        .iter()
        .map(|&x| thread::spawn(move || x * x))
        .collect();

    let results: Vec<i32> = handles.into_iter().map(|h| h.join().unwrap()).collect();

    let line: Vec<String> = results.iter().map(|r| r.to_string()).collect();
    println!("{}", line.join(" "));
}
