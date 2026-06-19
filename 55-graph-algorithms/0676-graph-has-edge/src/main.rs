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

    // Ask the library whether each edge is present.
    let bc = g.contains_edge("b", "c");
    let ae = g.contains_edge("a", "e");
    println!("{} {}", bc, ae);
}
