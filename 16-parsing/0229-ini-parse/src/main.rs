fn main() {
    let ini = "[s]\nk=v";
    let mut section = String::new();
    for line in ini.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        if let Some(name) = line.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            section = name.to_string();
        } else if let Some((key, val)) = line.split_once('=') {
            println!("{}.{}={}", section, key, val);
        }
    }
}
