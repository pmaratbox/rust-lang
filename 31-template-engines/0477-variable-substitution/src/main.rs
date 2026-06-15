use tera::{Context, Tera};

fn main() {
    let mut ctx = Context::new();
    ctx.insert("name", "alice");
    print!("{}", Tera::one_off("Hello {{ name }}", &ctx, false).unwrap());
    println!();
}
