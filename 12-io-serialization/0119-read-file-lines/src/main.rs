use std::fs;

fn main() {
    let path = "lines.txt";
    fs::write(path, "one\ntwo\nthree\n").unwrap();

    let contents = fs::read_to_string(path).unwrap();
    let count = contents.lines().filter(|l| !l.trim().is_empty()).count();

    fs::remove_file(path).unwrap();

    println!("lines: {}", count);
}
