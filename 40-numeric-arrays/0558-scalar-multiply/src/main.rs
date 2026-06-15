use ndarray::array;

fn main() {
    let a = array![[1, 2], [3, 4]];
    let scalar: i32 = 3;
    let c = &a * scalar;
    for row in c.rows() {
        println!(
            "{}",
            row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
        );
    }
}
