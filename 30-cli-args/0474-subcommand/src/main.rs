use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Add two integers.
    Add { a: i64, b: i64 },
}

fn main() {
    // Parse a hardcoded argv so the output is deterministic.
    let cli = Cli::parse_from(["prog", "add", "2", "3"]);
    match cli.command {
        Command::Add { a, b } => println!("{}", a + b),
    }
}
