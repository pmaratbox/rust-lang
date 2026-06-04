fn main() {
    let input = "alice,30\nbob,25";

    let pairs: Vec<String> = input
        .lines()
        .filter_map(|line| {
            let (name, value) = line.split_once(',')?;
            Some(format!("{}={}", name, value))
        })
        .collect();

    println!("{}", pairs.join(" "));
}
