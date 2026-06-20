use owo_colors::OwoColorize;

fn main() {
    // owo-colors' .black() always emits the raw ANSI escape (no TTY needed):
    // foreground BLACK is \x1b[30m and the foreground reset is \x1b[39m.
    println!("{}", "black".black());
}
