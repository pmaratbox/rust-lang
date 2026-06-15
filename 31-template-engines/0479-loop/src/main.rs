use tera::{Context, Tera};

fn main() {
    let mut c = Context::new();
    c.insert("nums", &[1, 2, 3]);
    let src = "{% for n in nums %}{{ n }}{% if not loop.last %}\n{% endif %}{% endfor %}";
    print!("{}", Tera::one_off(src, &c, false).unwrap());
    println!();
}
