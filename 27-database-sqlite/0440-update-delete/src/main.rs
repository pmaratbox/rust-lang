use rusqlite::{Connection, params};

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("create table users(id integer, name text)", []).unwrap();
    for (id, name) in [(1, "alice"), (2, "bob"), (3, "carol")] {
        conn.execute("insert into users values(?1, ?2)", params![id, name]).unwrap();
    }
    conn.execute("update users set name='robert' where id=2", []).unwrap();
    conn.execute("delete from users where id=1", []).unwrap();
    let mut stmt = conn.prepare("select id, name from users order by id").unwrap();
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, i64>(0)?, r.get::<_, String>(1)?)))
        .unwrap();
    for row in rows {
        let (id, name) = row.unwrap();
        println!("{} {}", id, name);
    }
}
