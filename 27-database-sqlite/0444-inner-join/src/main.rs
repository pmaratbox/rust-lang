use rusqlite::{Connection, params};

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("create table users(id integer, name text)", []).unwrap();
    conn.execute("create table orders(user_id integer, item text)", []).unwrap();
    for (id, name) in [(1, "alice"), (2, "bob")] {
        conn.execute("insert into users values(?1, ?2)", params![id, name]).unwrap();
    }
    for (user_id, item) in [(1, "book"), (2, "pen"), (1, "lamp")] {
        conn.execute("insert into orders values(?1, ?2)", params![user_id, item]).unwrap();
    }
    let mut stmt = conn
        .prepare("select u.name, o.item from orders o join users u on u.id = o.user_id order by u.name, o.item")
        .unwrap();
    let rows = stmt
        .query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?)))
        .unwrap();
    for row in rows {
        let (name, item) = row.unwrap();
        println!("{} {}", name, item);
    }
}
