use tera::{Context, Tera};

fn main() {
    let users = serde_json::json!([
        { "name": "alice", "age": 30 },
        { "name": "bob", "age": 25 },
    ]);

    let mut ctx = Context::new();
    ctx.insert("users", &users);

    let template = "{% for u in users %}{{ u.name }}: {{ u.age }}\n{% endfor %}";
    let rendered = Tera::one_off(template, &ctx, false).unwrap();

    print!("{}", rendered.trim_end_matches('\n'));
    println!();
}
