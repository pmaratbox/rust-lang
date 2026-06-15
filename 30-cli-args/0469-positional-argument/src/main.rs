use clap::Parser;

#[derive(Parser)]
struct Args {
    name: String,
}

fn main() {
    let args = Args::parse_from(["prog", "alice"]);
    println!("{}", args.name);
}
