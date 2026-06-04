fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

fn main() {
    let edges: [(usize, usize); 3] = [(0, 1), (1, 2), (3, 4)];
    let n = 5;
    let mut parent: Vec<usize> = (0..n).collect();
    for &(u, v) in &edges {
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru != rv {
            parent[ru] = rv;
        }
    }

    let count = (0..n).filter(|&x| find(&mut parent, x) == x).count();
    println!("{}", count);
}
