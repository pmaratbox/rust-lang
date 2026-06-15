use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    num: Vec<i64>,
}

fn main() {
    let a = Args::parse_from(["prog", "--num", "1", "--num", "2", "--num", "3"]);
    let sum: i64 = a.num.iter().sum();
    println!("{}", sum);
}
