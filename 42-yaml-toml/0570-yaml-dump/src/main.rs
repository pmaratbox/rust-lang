use std::collections::BTreeMap;

fn main() {
    // Build the map. A BTreeMap keeps keys in sorted order, and serde_yaml
    // emits block-style YAML (no flow braces, no quotes on simple scalars).
    let mut m: BTreeMap<&str, serde_yaml::Value> = BTreeMap::new();
    m.insert("name", "Alice".into());
    m.insert("age", 30.into());
    m.insert("city", "Paris".into());

    // serde_yaml produces "age: 30\ncity: Paris\nname: Alice\n".
    print!("{}", serde_yaml::to_string(&m).unwrap());
}
