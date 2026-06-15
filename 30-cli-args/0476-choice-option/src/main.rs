use clap::{Parser, ValueEnum};

#[derive(Clone, ValueEnum)]
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn name(&self) -> &'static str {
        match self {
            Color::Red => "red",
            Color::Green => "green",
            Color::Blue => "blue",
        }
    }
}

#[derive(Parser)]
struct Args {
    #[arg(long, value_enum)]
    color: Color,
}

fn main() {
    let args = Args::parse_from(["prog", "--color", "green"]);
    println!("{}", args.color.name());
}
