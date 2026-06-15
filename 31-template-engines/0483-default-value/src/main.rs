use tera::{Tera, Context};

fn main() {
    // Data has NO `name` value, so the `default` filter supplies "anonymous".
    let c = Context::new();
    let src = "{{ name | default(value=\"anonymous\") }}";
    print!("{}", Tera::one_off(src, &c, false).unwrap());
    println!();
}
