fn main() {
    let data = "name,age,city\nAlice,30,Paris\nBob,25,London\nCarol,35,Berlin\n";

    // has_headers(false) so row 0 is the header and we control the slicing.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let rows: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();

    // Skip the header row; take the first column (name) of each data row.
    let names: Vec<&str> = rows[1..].iter().map(|r| r.get(0).unwrap()).collect();
    println!("{}", names.join(","));
}
