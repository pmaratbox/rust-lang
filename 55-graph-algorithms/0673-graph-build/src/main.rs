// petgraph — build the fixed weighted undirected graph G, then report its size.
use petgraph::graphmap::UnGraphMap;

fn main() {
    // Weighted undirected graph G: nodes a,b,c,d,e.
    let g: UnGraphMap<&str, i32> = UnGraphMap::from_edges(&[
        ("a", "b", 1),
        ("a", "c", 4),
        ("b", "c", 1),
        ("b", "d", 5),
        ("c", "d", 1),
        ("d", "e", 1),
    ]);

    // Node count and edge count, space-joined, straight from the library.
    println!("{} {}", g.node_count(), g.edge_count());
}
