fn main() {
    let data = "name,age,city\nAlice,30,Paris\nBob,25,London\nCarol,35,Berlin\n";

    // has_headers(false) so row 0 is the header and we control the slicing.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let rows: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();

    // Find the column index named "age" in the header row.
    let age_idx = rows[0]
        .iter()
        .position(|h| h == "age")
        .expect("age column");

    // Extract that column from each data row and join with commas.
    let ages: Vec<&str> = rows[1..].iter().map(|r| r.get(age_idx).unwrap()).collect();
    println!("{}", ages.join(","));
}
