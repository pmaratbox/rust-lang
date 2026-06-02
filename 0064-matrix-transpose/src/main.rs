fn main() {
    let matrix = [[1, 2, 3], [4, 5, 6]];
    let rows = matrix.len();
    let cols = matrix[0].len();

    let mut transposed = vec![vec![0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            transposed[j][i] = matrix[i][j];
        }
    }

    for row in &transposed {
        let parts: Vec<String> = row.iter().map(|x| x.to_string()).collect();
        println!("{}", parts.join(" "));
    }
}
