use diesel::dsl::sum;
use diesel::prelude::*;

diesel::table! {
    products (id) {
        id -> Integer,
        category -> Text,
        price -> Integer,
    }
}

#[derive(Insertable)]
#[diesel(table_name = products)]
struct NewProduct {
    id: i32,
    category: String,
    price: i32,
}

fn main() {
    let mut c = SqliteConnection::establish(":memory:").unwrap();
    diesel::sql_query("create table products(id integer primary key, category text, price integer)")
        .execute(&mut c)
        .unwrap();

    let rows = vec![
        NewProduct { id: 1, category: "a".into(), price: 10 },
        NewProduct { id: 2, category: "b".into(), price: 20 },
        NewProduct { id: 3, category: "a".into(), price: 30 },
    ];
    diesel::insert_into(products::table)
        .values(&rows)
        .execute(&mut c)
        .unwrap();

    let groups: Vec<(String, Option<i64>)> = products::table
        .group_by(products::category)
        .select((products::category, sum(products::price)))
        .order(products::category)
        .load(&mut c)
        .unwrap();

    for (category, total) in groups {
        println!("{} {}", category, total.unwrap());
    }
}
