use pulldown_cmark::{Parser, html};

fn render(src: &str) -> String {
    let mut out = String::new();
    html::push_html(&mut out, Parser::new(src));
    out.trim_end_matches('\n').to_string()
}

fn main() {
    print!("{}", render("- a\n- b"));
    println!();
}
