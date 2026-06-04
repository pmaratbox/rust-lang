struct SegTree {
    n: usize,
    tree: Vec<i64>,
}

impl SegTree {
    fn new(data: &[i64]) -> Self {
        let n = data.len();
        let mut tree = vec![0; 2 * n];
        // Leaves.
        for (i, &v) in data.iter().enumerate() {
            tree[n + i] = v;
        }
        // Internal nodes.
        for i in (1..n).rev() {
            tree[i] = tree[2 * i] + tree[2 * i + 1];
        }
        SegTree { n, tree }
    }

    // Sum over [l, r] inclusive.
    fn query(&self, l: usize, r: usize) -> i64 {
        let mut lo = l + self.n;
        let mut hi = r + self.n + 1;
        let mut sum = 0;
        while lo < hi {
            if lo & 1 == 1 {
                sum += self.tree[lo];
                lo += 1;
            }
            if hi & 1 == 1 {
                hi -= 1;
                sum += self.tree[hi];
            }
            lo >>= 1;
            hi >>= 1;
        }
        sum
    }
}

fn main() {
    let st = SegTree::new(&[1, 2, 3, 4, 5]);
    println!("{}", st.query(1, 3));
}
