use diesel::prelude::*;

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        age -> Integer,
    }
}

#[derive(Queryable)]
#[allow(dead_code)]
struct User {
    id: i32,
    name: String,
    age: i32,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    id: i32,
    name: String,
    age: i32,
}

fn main() {
    let mut c = SqliteConnection::establish(":memory:").unwrap();
    diesel::sql_query("create table users(id integer primary key, name text, age integer)")
        .execute(&mut c)
        .unwrap();

    let rows = vec![
        NewUser { id: 1, name: "alice".into(), age: 30 },
        NewUser { id: 2, name: "bob".into(), age: 25 },
        NewUser { id: 3, name: "carol".into(), age: 35 },
    ];
    diesel::insert_into(users::table)
        .values(&rows)
        .execute(&mut c)
        .unwrap();

    diesel::delete(users::table.filter(users::id.eq(1)))
        .execute(&mut c)
        .unwrap();

    let result = users::table
        .order(users::id)
        .load::<User>(&mut c)
        .unwrap();

    for u in result {
        println!("{}", u.name);
    }
}
