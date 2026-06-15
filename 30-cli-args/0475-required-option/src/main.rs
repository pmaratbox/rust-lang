use clap::Parser;

#[derive(Parser)]
struct Args {
    // Marked required: clap errors if --id is missing from the argv.
    #[arg(long, required = true)]
    id: i64,
}

fn main() {
    // Parse a hardcoded argv (not real process args) for deterministic output.
    let args = Args::parse_from(["prog", "--id", "42"]);
    println!("{}", args.id);
}
