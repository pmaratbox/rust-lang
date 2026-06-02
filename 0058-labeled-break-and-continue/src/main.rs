fn main() {
    'outer: for i in 1..=3 {
        for j in 1..=3 {
            if j > i {
                continue 'outer;
            }
            if i * j == 4 {
                println!("stop at {},{}", i, j);
                break 'outer;
            }
            println!("{},{}", i, j);
        }
    }
}
