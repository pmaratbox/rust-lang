use std::collections::BTreeMap;

fn main() {
    // Parse a YAML document with a nested `server` mapping using serde_yaml.
    let doc = "server:\n  host: localhost\n  port: 8080\n";
    let d: BTreeMap<String, serde_yaml::Value> = serde_yaml::from_str(doc).unwrap();

    let server = &d["server"];
    let host = server["host"].as_str().unwrap();
    let port = server["port"].as_i64().unwrap();

    println!("{}:{}", host, port);
}
