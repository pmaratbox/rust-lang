fn main() {
    // Parse a TOML document containing an array of strings.
    let doc: toml::Value = "tags = [\"red\", \"green\", \"blue\"]\n".parse().unwrap();

    // Read the `tags` array and collect each element as a string.
    let tags: Vec<String> = doc["tags"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();

    // Join with commas: red,green,blue
    println!("{}", tags.join(","));
}
