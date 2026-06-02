fn main() {
    let a = [[1, 2], [3, 4]];
    let b = [[5, 6], [7, 8]];
    let n = 2;
    let mut result = [[0; 2]; 2];
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }

    for row in &result {
        let parts: Vec<String> = row.iter().map(|x| x.to_string()).collect();
        println!("{}", parts.join(" "));
    }
}
