fn find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = find(parent, parent[x]);
    }
    parent[x]
}

fn main() {
    let mut edges: Vec<(usize, usize, u32)> = vec![(0, 1, 1), (1, 2, 2), (0, 2, 3)];
    let n = 3;
    edges.sort_by_key(|&(_, _, w)| w);

    let mut parent: Vec<usize> = (0..n).collect();
    let mut total = 0;
    for (u, v, w) in edges {
        let ru = find(&mut parent, u);
        let rv = find(&mut parent, v);
        if ru != rv {
            parent[ru] = rv;
            total += w;
        }
    }

    println!("{}", total);
}
