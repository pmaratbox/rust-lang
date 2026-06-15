use rusqlite::{Connection, params};

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("create table inv(item text primary key, qty integer)", [])
        .unwrap();
    conn.execute("insert into inv values(?1, ?2)", params!["apple", 5])
        .unwrap();

    let upsert = "insert into inv(item, qty) values(?1, ?2) \
                  on conflict(item) do update set qty = qty + excluded.qty";
    conn.execute(upsert, params!["apple", 5]).unwrap();
    conn.execute(upsert, params!["banana", 3]).unwrap();

    let mut stmt = conn.prepare("select item, qty from inv order by item").unwrap();
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))
        .unwrap();
    for row in rows {
        let (item, qty) = row.unwrap();
        println!("{} {}", item, qty);
    }
}
