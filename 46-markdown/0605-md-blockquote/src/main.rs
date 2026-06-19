use pulldown_cmark::{html, Parser};

fn render(src: &str) -> String {
    let mut out = String::new();
    html::push_html(&mut out, Parser::new(src));
    out.trim_end_matches('\n').to_string()
}

fn main() {
    // A blockquote: `> quote` renders as a <blockquote> wrapping a paragraph.
    print!("{}", render("> quote"));
    println!();
}
