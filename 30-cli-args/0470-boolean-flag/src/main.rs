use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    verbose: bool,
}

fn main() {
    // Parse a hardcoded argv so the output is deterministic.
    let args = Args::parse_from(["prog", "--verbose"]);
    println!("{}", args.verbose);
}
