use std::collections::BTreeMap;

fn main() {
    let words = ["one", "two", "three"];
    let mut groups: BTreeMap<usize, Vec<&str>> = BTreeMap::new();
    for w in words {
        groups.entry(w.len()).or_default().push(w);
    }

    let out: Vec<String> = groups
        .iter()
        .map(|(k, ws)| format!("{}:[{}]", k, ws.join(",")))
        .collect();
    println!("{}", out.join(" "));
}
