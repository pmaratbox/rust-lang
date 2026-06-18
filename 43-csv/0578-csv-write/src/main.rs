fn main() {
    // Build CSV with the csv crate's writer instead of formatting strings by hand.
    let mut wtr = csv::WriterBuilder::new().from_writer(vec![]);
    wtr.write_record(["name", "age"]).unwrap();
    wtr.write_record(["Alice", "30"]).unwrap();

    // Recover the written bytes, then normalize CRLF -> LF and drop the
    // trailing newline so the output is byte-identical across platforms.
    let bytes = wtr.into_inner().unwrap();
    let text = String::from_utf8(bytes).unwrap();
    let normalized = text.replace("\r\n", "\n");
    println!("{}", normalized.trim_end_matches('\n'));
}
