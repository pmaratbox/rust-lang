fn main() {
    // Parse a TOML document with a [server] table using the toml crate.
    let doc = "[server]\nhost = \"localhost\"\nport = 8080\n";
    let t: toml::Value = doc.parse().unwrap();

    let host = t["server"]["host"].as_str().unwrap();
    let port = t["server"]["port"].as_integer().unwrap();

    println!("host={} port={}", host, port);
}
