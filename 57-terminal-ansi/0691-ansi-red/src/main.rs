use owo_colors::OwoColorize;

fn main() {
    // owo-colors' `.red()` always emits the raw ANSI escape, with no TTY
    // detection, so color is effectively forced on. The foreground reset it
    // writes is \x1b[39m (default fg), not the full \x1b[0m reset.
    println!("{}", "red".red()); // \x1b[31mred\x1b[39m
}
