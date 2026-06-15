use rusqlite::{params, Connection};

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("create table users(id integer, name text)", [])
        .unwrap();
    for (id, name) in [(1, "alice"), (2, "bob"), (3, "carol")] {
        conn.execute("insert into users values(?1, ?2)", params![id, name])
            .unwrap();
    }
    let mut stmt = conn.prepare("select name from users order by id").unwrap();
    let rows = stmt.query_map([], |r| r.get::<_, String>(0)).unwrap();
    for row in rows {
        println!("{}", row.unwrap());
    }
}
