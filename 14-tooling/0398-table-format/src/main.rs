fn main() {
    let rows = [("a", "1"), ("bb", "22")];
    let width = rows.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
    for (col1, col2) in rows {
        println!("{:<width$} | {}", col1, col2, width = width);
    }
}
