// Rust — petgraph. Degree of a node = number of incident edges.
use petgraph::graphmap::UnGraphMap;

fn main() {
    // The fixed weighted undirected graph G.
    let g: UnGraphMap<&str, i32> = UnGraphMap::from_edges(&[
        ("a", "b", 1),
        ("a", "c", 4),
        ("b", "c", 1),
        ("b", "d", 5),
        ("c", "d", 1),
        ("d", "e", 1),
    ]);

    // Degree of node b: count its incident neighbors via the library.
    let degree = g.neighbors("b").count();
    println!("{}", degree);
}
