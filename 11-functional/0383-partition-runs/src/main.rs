fn main() {
    let data = [1, 1, 2, 3, 3, 3];
    let mut runs: Vec<Vec<i32>> = Vec::new();
    for &x in &data {
        match runs.last_mut() {
            Some(run) if *run.last().unwrap() == x => run.push(x),
            _ => runs.push(vec![x]),
        }
    }
    let out = runs
        .iter()
        .map(|run| {
            run.iter()
                .map(|n| n.to_string())
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect::<Vec<_>>()
        .join("|");
    println!("{}", out);
}
