use rusqlite::{Connection, params};

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("create table users(id integer, name text)", []).unwrap();
    for (id, name) in [(1, "alice"), (2, "bob"), (3, "carol")] {
        conn.execute("insert into users values(?1, ?2)", params![id, name]).unwrap();
    }
    let mut stmt = conn.prepare("select name from users where id = ?1").unwrap();
    let name: String = stmt.query_row(params![2], |r| r.get::<_, String>(0)).unwrap();
    println!("{}", name);
}
