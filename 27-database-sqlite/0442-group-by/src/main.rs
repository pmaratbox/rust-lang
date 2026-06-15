use rusqlite::{Connection, params};
fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("create table sales(category text, amount integer)", []).unwrap();
    for (category, amount) in [("a", 10), ("b", 20), ("a", 30), ("b", 5), ("a", 2)] {
        conn.execute("insert into sales values(?1, ?2)", params![category, amount]).unwrap();
    }
    let mut stmt = conn
        .prepare("select category, sum(amount) from sales group by category order by category")
        .unwrap();
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))
        .unwrap();
    for row in rows {
        let (category, total) = row.unwrap();
        println!("{} {}", category, total);
    }
}
