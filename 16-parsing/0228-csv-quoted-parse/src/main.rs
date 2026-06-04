fn main() {
    let row = r#"a,"b,c",d"#;
    let mut fields: Vec<String> = Vec::new();
    let mut cur = String::new();
    let mut in_quotes = false;
    for ch in row.chars() {
        match ch {
            '"' => in_quotes = !in_quotes,
            ',' if !in_quotes => {
                fields.push(std::mem::take(&mut cur));
            }
            _ => cur.push(ch),
        }
    }
    fields.push(cur);
    println!("{}", fields.join("|"));
}
