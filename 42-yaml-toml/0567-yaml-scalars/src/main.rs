use std::collections::BTreeMap;

fn main() {
    // Parse a YAML mapping with serde_yaml, then read the scalar fields.
    let doc = "name: Alice\nrole: admin\nage: 30\n";
    let d: BTreeMap<String, serde_yaml::Value> = serde_yaml::from_str(doc).unwrap();

    let name = d["name"].as_str().unwrap();
    let role = d["role"].as_str().unwrap();
    let age = d["age"].as_i64().unwrap();

    println!("{} {} {}", name, role, age);
}
