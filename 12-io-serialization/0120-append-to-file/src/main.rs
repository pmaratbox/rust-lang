use std::fs::{self, OpenOptions};
use std::io::Write;

fn main() {
    let path = "append.txt";

    fs::write(path, "a\n").unwrap();

    let mut file = OpenOptions::new().append(true).open(path).unwrap();
    writeln!(file, "b").unwrap();

    let contents = fs::read_to_string(path).unwrap();
    let joined = contents
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    fs::remove_file(path).unwrap();

    println!("{}", joined);
}
