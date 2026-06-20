use owo_colors::OwoColorize;

fn main() {
    // owo-colors' .white() always emits the raw ANSI escape (no TTY needed):
    // \x1b[37m … \x1b[39m, where \x1b[39m is the foreground-reset code.
    println!("{}", "white".white());
}
