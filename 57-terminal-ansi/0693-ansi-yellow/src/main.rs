use owo_colors::OwoColorize;

fn main() {
    // owo-colors' .yellow() always emits the raw ANSI escape (no TTY needed):
    // \x1b[33m then the text then the foreground reset \x1b[39m.
    println!("{}", "yellow".yellow());
}
