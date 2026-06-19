use std::collections::BTreeMap;

fn hexs(b: &[u8]) -> String {
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

fn main() {
    // MessagePack-encode the single-key map {"a": 1}. A BTreeMap keeps key
    // order deterministic. The encoding is a fixmap header (0x81) followed by
    // the key as fixstr ("a" -> a161) and the value as positive fixint (01).
    let mut m: BTreeMap<&str, i32> = BTreeMap::new();
    m.insert("a", 1);
    let bytes = rmp_serde::to_vec(&m).unwrap();
    println!("{}", hexs(&bytes));
}
