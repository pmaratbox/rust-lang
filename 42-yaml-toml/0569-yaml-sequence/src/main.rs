use std::collections::BTreeMap;

fn main() {
    // Parse a YAML document whose `fruits` key holds a sequence (list).
    let doc = "fruits:\n  - apple\n  - banana\n  - cherry\n";
    let d: BTreeMap<String, serde_yaml::Value> = serde_yaml::from_str(doc).unwrap();

    // Pull the sequence out and join the string items with commas.
    let fruits: Vec<String> = d["fruits"]
        .as_sequence()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect();

    println!("{}", fruits.join(","));
}
