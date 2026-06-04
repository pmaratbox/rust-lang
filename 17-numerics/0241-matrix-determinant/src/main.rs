fn main() {
    let m = [[1i64, 2], [3, 4]];
    let det = m[0][0] * m[1][1] - m[0][1] * m[1][0];
    println!("{}", det);
}
