fn sum(xs: &[i32]) -> i32 {
    match xs {
        [] => 0,
        [head, tail @ ..] => head + sum(tail),
    }
}

fn main() {
    println!("{}", sum(&[1, 2, 3, 4]));
}
