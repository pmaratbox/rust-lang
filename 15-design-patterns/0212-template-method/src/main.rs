trait Algorithm {
    fn step(&self) -> &str;
    fn run(&self) -> String {
        format!("start {} end", self.step())
    }
}

struct ConcreteAlgorithm;
impl Algorithm for ConcreteAlgorithm {
    fn step(&self) -> &str {
        "work"
    }
}

fn main() {
    let algo = ConcreteAlgorithm;
    println!("{}", algo.run());
}
