// Parse a TOML array-of-tables ([[servers]]) with the `toml` crate,
// collect each server's name, and join them with commas.

fn main() {
    let doc: toml::Value =
        "[[servers]]\nname = \"alpha\"\n[[servers]]\nname = \"beta\"\n"
            .parse()
            .unwrap();
    let names: Vec<String> = doc["servers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["name"].as_str().unwrap().to_string())
        .collect();
    println!("{}", names.join(","));
}
