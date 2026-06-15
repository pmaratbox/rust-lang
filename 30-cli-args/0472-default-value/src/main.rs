use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value_t = 1)]
    count: i32,
}

fn main() {
    // Hardcoded EMPTY argv (just the program name) for deterministic output:
    // --count is absent, so clap supplies its default value of 1.
    let a = Args::parse_from(["prog"]);
    println!("{}", a.count);
}
