use std::env;
use std::fs;

fn main() {
    let path = env::temp_dir().join("roundtrip.txt");

    let data = "ok-data";
    fs::write(&path, data).unwrap();

    let read_back = fs::read_to_string(&path).unwrap();

    fs::remove_file(&path).unwrap();

    let status = if read_back == data { "ok" } else { "mismatch" };
    println!("roundtrip: {}", status);
}
