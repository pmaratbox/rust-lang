fn main() {
    let targets = [2, 0];
    let mut floor: i32 = 0;
    let mut out = vec![floor];
    for target in targets {
        while floor != target {
            floor += if target > floor { 1 } else { -1 };
            out.push(floor);
        }
    }
    let line: Vec<String> = out.iter().map(|f| f.to_string()).collect();
    println!("{}", line.join(" "));
}
