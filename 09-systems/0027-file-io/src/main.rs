use std::fs;

fn main() {
    let path = "greeting.txt";
    fs::write(path, "hello, file").unwrap();
    let content = fs::read_to_string(path).unwrap();
    fs::remove_file(path).unwrap();
    println!("read: {content}");
}
