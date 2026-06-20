// Rust — owo-colors. The .green() adaptor always emits the raw ANSI
// escape (foreground GREEN = \x1b[32m, foreground reset = \x1b[39m),
// regardless of whether stdout is a TTY.
use owo_colors::OwoColorize;

fn main() {
    println!("{}", "green".green());
}
