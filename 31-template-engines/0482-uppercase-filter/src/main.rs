use tera::{Context, Tera};

fn main() {
    let mut ctx = Context::new();
    ctx.insert("name", "alice");
    print!("{}", Tera::one_off("{{ name | upper }}", &ctx, false).unwrap());
    println!();
}
