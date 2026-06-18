// Parse top-level TOML scalars with the `toml` crate and print them space-joined.

fn main() {
    let doc: toml::Value = "title = \"demo\"\nversion = 2\n".parse().unwrap();
    let title = doc["title"].as_str().unwrap();
    let version = doc["version"].as_integer().unwrap();
    println!("{} {}", title, version);
}
