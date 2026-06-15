use diesel::prelude::*;

diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    posts (id) {
        id -> Integer,
        user_id -> Integer,
        title -> Text,
    }
}

diesel::joinable!(posts -> users (user_id));
diesel::allow_tables_to_appear_in_same_query!(users, posts);

#[derive(Insertable)]
#[diesel(table_name = users)]
struct NewUser {
    id: i32,
    name: String,
}

#[derive(Insertable)]
#[diesel(table_name = posts)]
struct NewPost {
    id: i32,
    user_id: i32,
    title: String,
}

fn main() {
    let mut c = SqliteConnection::establish(":memory:").unwrap();
    diesel::sql_query("create table users(id integer primary key, name text not null)")
        .execute(&mut c)
        .unwrap();
    diesel::sql_query(
        "create table posts(id integer primary key, user_id integer not null, title text not null)",
    )
    .execute(&mut c)
    .unwrap();

    let user_rows = vec![
        NewUser { id: 1, name: "alice".into() },
        NewUser { id: 2, name: "bob".into() },
    ];
    diesel::insert_into(users::table)
        .values(&user_rows)
        .execute(&mut c)
        .unwrap();

    let post_rows = vec![
        NewPost { id: 1, user_id: 1, title: "hello".into() },
        NewPost { id: 2, user_id: 1, title: "world".into() },
        NewPost { id: 3, user_id: 2, title: "hi".into() },
    ];
    diesel::insert_into(posts::table)
        .values(&post_rows)
        .execute(&mut c)
        .unwrap();

    let rows: Vec<(String, String)> = posts::table
        .inner_join(users::table)
        .select((users::name, posts::title))
        .order((users::name, posts::title))
        .load(&mut c)
        .unwrap();

    for (name, title) in rows {
        println!("{} {}", name, title);
    }
}
