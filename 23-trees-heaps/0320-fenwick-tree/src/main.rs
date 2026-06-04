struct Fenwick {
    tree: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { tree: vec![0; n + 1] }
    }

    fn update(&mut self, mut i: usize, delta: i64) {
        while i < self.tree.len() {
            self.tree[i] += delta;
            i += i & i.wrapping_neg();
        }
    }

    // Prefix sum of the first i elements (1-indexed).
    fn prefix(&self, mut i: usize) -> i64 {
        let mut sum = 0;
        while i > 0 {
            sum += self.tree[i];
            i -= i & i.wrapping_neg();
        }
        sum
    }
}

fn main() {
    let data = [1, 2, 3, 4, 5];
    let mut bit = Fenwick::new(data.len());
    for (i, &v) in data.iter().enumerate() {
        bit.update(i + 1, v);
    }
    println!("{}", bit.prefix(4));
}
