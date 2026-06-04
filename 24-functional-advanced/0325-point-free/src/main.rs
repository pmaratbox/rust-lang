fn main() {
    let sum_of_squares = |xs: &[i32]| xs.iter().map(|x| x * x).sum::<i32>();
    println!("{}", sum_of_squares(&[1, 2, 3]));
}
