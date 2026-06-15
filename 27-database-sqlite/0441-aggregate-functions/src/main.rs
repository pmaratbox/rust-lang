use rusqlite::{Connection, params};

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("create table t(amount integer)", []).unwrap();
    for amount in [10, 20, 30, 40, 50] {
        conn.execute("insert into t values(?1)", params![amount]).unwrap();
    }
    let mut stmt = conn
        .prepare("select count(*), sum(amount), min(amount), max(amount) from t")
        .unwrap();
    let (count, sum, min, max) = stmt
        .query_row([], |r| {
            Ok((
                r.get::<_, i64>(0)?,
                r.get::<_, i64>(1)?,
                r.get::<_, i64>(2)?,
                r.get::<_, i64>(3)?,
            ))
        })
        .unwrap();
    println!("{}", count);
    println!("{}", sum);
    println!("{}", min);
    println!("{}", max);
}
