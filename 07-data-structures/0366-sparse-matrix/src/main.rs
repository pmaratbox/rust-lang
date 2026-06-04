use std::collections::HashMap;

struct SparseMatrix {
    entries: HashMap<(usize, usize), i32>,
}

impl SparseMatrix {
    fn new() -> Self {
        SparseMatrix { entries: HashMap::new() }
    }

    fn set(&mut self, row: usize, col: usize, value: i32) {
        if value == 0 {
            self.entries.remove(&(row, col));
        } else {
            self.entries.insert((row, col), value);
        }
    }

    fn get(&self, row: usize, col: usize) -> i32 {
        *self.entries.get(&(row, col)).unwrap_or(&0)
    }
}

fn main() {
    let mut matrix = SparseMatrix::new();
    matrix.set(1, 1, 5);
    println!("{} {}", matrix.get(1, 1), matrix.get(0, 0));
}
