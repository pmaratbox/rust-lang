use ndarray::array;

fn main() {
    let a = array![[1, 2, 3], [4, 5, 6]];
    let t = a.t();
    for row in t.rows() {
        println!(
            "{}",
            row.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
        );
    }
}
