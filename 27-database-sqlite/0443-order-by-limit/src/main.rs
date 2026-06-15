use rusqlite::{Connection, params};
fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("create table scores(value integer)", []).unwrap();
    for value in [50, 90, 70, 30, 100, 20] {
        conn.execute("insert into scores values(?1)", params![value]).unwrap();
    }
    let mut stmt = conn.prepare("select value from scores order by value desc limit 3").unwrap();
    let rows = stmt.query_map([], |r| r.get::<_, i64>(0)).unwrap();
    for row in rows {
        println!("{}", row.unwrap());
    }
}
