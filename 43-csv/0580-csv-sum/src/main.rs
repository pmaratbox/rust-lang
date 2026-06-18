fn main() {
    let data = "name,age,city\nAlice,30,Paris\nBob,25,London\nCarol,35,Berlin\n";
    // has_headers(false) so row 0 is the header and we control slicing:
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(data.as_bytes());
    let rows: Vec<csv::StringRecord> = rdr.records().map(|r| r.unwrap()).collect();
    // The `age` column is index 1; parse each data row to i64 and sum.
    let sum: i64 = rows[1..]
        .iter()
        .map(|r| r.get(1).unwrap().parse::<i64>().unwrap())
        .sum();
    println!("{}", sum); // 30 + 25 + 35 = 90
}
