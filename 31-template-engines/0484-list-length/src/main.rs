use tera::{Context, Tera};

fn main() {
    // Fixed data: a list of three items.
    let mut ctx = Context::new();
    ctx.insert("items", &serde_json::json!([1, 2, 3]));

    // Fixed template: Tera's `length` filter returns the number of elements.
    let src = "{{ items | length }}";

    print!("{}", Tera::one_off(src, &ctx, false).unwrap());
    println!();
}
