use owo_colors::OwoColorize;

fn main() {
    // owo-colors' `.blue()` always emits the raw ANSI escape for the
    // standard foreground BLUE color (ESC[34m ... ESC[39m), no TTY needed.
    println!("{}", "blue".blue());
}
