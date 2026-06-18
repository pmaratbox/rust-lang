fn main() {
    let data = "name,age,city\nAlice,30,Paris\nBob,25,London\nCarol,35,Berlin\n";
    // has_headers(false) so row 0 is the header row and we control slicing.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let rows: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();
    // Read the header (first) row's fields and join them with a pipe.
    let header = rows[0].iter().collect::<Vec<_>>().join("|");
    println!("{}", header); // name|age|city
}
