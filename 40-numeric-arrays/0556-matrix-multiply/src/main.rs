use ndarray::array;

fn main() {
    let a = array![[1, 2], [3, 4]];
    let b = array![[5, 6], [7, 8]];
    let c = a.dot(&b);
    for row in c.rows() {
        let line = row
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("{}", line);
    }
}
