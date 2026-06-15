use tera::{Context, Tera};

fn main() {
    let mut ctx = Context::new();
    ctx.insert("user", &serde_json::json!({ "name": "alice" }));
    print!("{}", Tera::one_off("{{ user.name }}", &ctx, false).unwrap());
    println!();
}
