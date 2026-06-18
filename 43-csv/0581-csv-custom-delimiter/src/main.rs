fn main() {
    let data = "a;b;c\n1;2;3\n";
    // Configure the csv crate to treat ';' as the field delimiter.
    // has_headers(false) keeps row 0 as the header so we control slicing.
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_reader(data.as_bytes());
    let rows: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();
    // Take the second (data) row and join its fields with commas.
    let fields: Vec<&str> = rows[1].iter().collect();
    println!("{}", fields.join(","));
}
