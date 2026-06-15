use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    name: String,
}

fn main() {
    // Parse a hardcoded argv (not real process args) for deterministic output.
    let args = Args::parse_from(["prog", "--name", "alice"]);
    println!("{}", args.name);
}
