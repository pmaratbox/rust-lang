use rusqlite::{Connection, params};

fn main() {
    let mut conn = Connection::open_in_memory().unwrap();
    conn.execute("create table t(n integer)", []).unwrap();

    // Insert 1000 rows efficiently: one transaction, one prepared statement
    // reused for every row binding.
    {
        let tx = conn.transaction().unwrap();
        {
            let mut stmt = tx.prepare("insert into t values(?1)").unwrap();
            for n in 1..=1000 {
                stmt.execute(params![n]).unwrap();
            }
        }
        tx.commit().unwrap();
    }

    let count: i64 = conn
        .query_row("select count(*) from t", [], |r| r.get(0))
        .unwrap();
    println!("{}", count);
}
