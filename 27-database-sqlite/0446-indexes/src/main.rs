use rusqlite::{Connection, params};

fn main() {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("create table products(id integer, sku text, price integer)", []).unwrap();
    for (id, sku, price) in [(1, "A", 100), (2, "B", 200), (3, "C", 300)] {
        conn.execute("insert into products values(?1, ?2, ?3)", params![id, sku, price]).unwrap();
    }
    conn.execute("create index idx_sku on products(sku)", []).unwrap();
    let mut stmt = conn.prepare("select price from products where sku=?").unwrap();
    let price: i64 = stmt.query_row(params!["B"], |r| r.get(0)).unwrap();
    println!("{}", price);
}
