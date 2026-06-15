use rusqlite::Connection;

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    let answer: i64 = conn
        .query_row("select 42", [], |r| r.get(0))
        .unwrap();
    println!("{}", answer);
}
