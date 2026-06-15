use serde_json::Value;

fn main() {
    let input = r#"{"user":{"name":"alice","roles":["admin","user"]}}"#;

    // Parse into serde_json's dynamic tree (Value), then index into the
    // nested structure rather than deserializing into typed structs.
    let root: Value = serde_json::from_str(input).unwrap();

    let name = root["user"]["name"].as_str().unwrap();
    let first_role = root["user"]["roles"][0].as_str().unwrap();

    println!("{} {}", name, first_role);
}
