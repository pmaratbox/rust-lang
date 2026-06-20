use owo_colors::OwoColorize;

fn main() {
    // owo-colors' .cyan() always emits the raw ANSI escape (no TTY needed):
    // \x1b[36m … \x1b[39m, where \x1b[39m is the foreground-reset code.
    println!("{}", "cyan".cyan());
}
