fn main() {
    let data = "name,age,city\nAlice,30,Paris\nBob,25,London\nCarol,35,Berlin\n";

    // has_headers(false) so row 0 is the header and we control the slicing.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let rows: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();

    // Skip the header; keep data rows whose age (column 1) is > 28.
    let kept: Vec<&str> = rows[1..]
        .iter()
        .filter(|r| r.get(1).unwrap().parse::<i64>().unwrap() > 28)
        .map(|r| r.get(0).unwrap())
        .collect();
    println!("{}", kept.join(","));
}
