use rusqlite::Connection;

fn main() {
    let mut conn = Connection::open_in_memory().unwrap();
    conn.execute("create table t(n integer)", []).unwrap();

    // First transaction: insert 1 and 2, then commit.
    {
        let tx = conn.transaction().unwrap();
        tx.execute("insert into t values(1)", []).unwrap();
        tx.execute("insert into t values(2)", []).unwrap();
        tx.commit().unwrap();
    }

    // Second transaction: insert 3, then roll back (discard the change).
    {
        let tx = conn.transaction().unwrap();
        tx.execute("insert into t values(3)", []).unwrap();
        tx.rollback().unwrap();
    }

    let mut stmt = conn.prepare("select n from t order by n").unwrap();
    let rows = stmt.query_map([], |r| r.get::<_, i64>(0)).unwrap();
    for row in rows {
        println!("{}", row.unwrap());
    }
}
