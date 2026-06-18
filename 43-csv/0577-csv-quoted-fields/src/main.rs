fn main() {
    let data = "name,note\nAlice,\"hello, world\"\n";
    // has_headers(false) so row 0 is the header and row 1 is the data row.
    // The reader's default quoting rules unwrap "hello, world" into one field,
    // keeping the embedded comma instead of splitting on it.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let rows: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();
    // Print the `note` value (column 1) of the data row.
    let note = rows[1].get(1).unwrap();
    println!("{}", note); // hello, world
}
