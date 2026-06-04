use std::collections::HashMap;

fn main() {
    let template = "hi {name}";
    let mut vars: HashMap<&str, &str> = HashMap::new();
    vars.insert("name", "Ada");

    let mut out = String::new();
    let mut rest = template;
    while let Some(start) = rest.find('{') {
        out.push_str(&rest[..start]);
        let end = rest[start..].find('}').unwrap() + start;
        let key = &rest[start + 1..end];
        out.push_str(vars[key]);
        rest = &rest[end + 1..];
    }
    out.push_str(rest);
    println!("{}", out);
}
