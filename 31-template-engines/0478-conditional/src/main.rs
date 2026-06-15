use tera::{Tera, Context};

fn main() {
    let mut c = Context::new();
    c.insert("logged_in", &true);
    let src = "{% if logged_in %}welcome{% else %}guest{% endif %}";
    print!("{}", Tera::one_off(src, &c, false).unwrap());
    println!();
}
