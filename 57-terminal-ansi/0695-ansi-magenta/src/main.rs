// owo-colors — color the word "magenta" with foreground MAGENTA (ANSI 35).
// The `.magenta()` extension method wraps the value so its Display impl emits
// the raw escape: \x1b[35m ... \x1b[39m (foreground reset, not \x1b[0m).
use owo_colors::OwoColorize;

fn main() {
    println!("{}", "magenta".magenta());
}
