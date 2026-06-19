use petgraph::graphmap::UnGraphMap;

fn main() {
    // Fixed weighted undirected graph G.
    let g: UnGraphMap<&str, i32> = UnGraphMap::from_edges(&[
        ("a", "b", 1),
        ("a", "c", 4),
        ("b", "c", 1),
        ("b", "d", 5),
        ("c", "d", 1),
        ("d", "e", 1),
    ]);

    // Neighbors of node "a", sorted for determinism, comma-joined.
    let mut nb: Vec<&str> = g.neighbors("a").collect();
    nb.sort();
    println!("{}", nb.join(","));
}
