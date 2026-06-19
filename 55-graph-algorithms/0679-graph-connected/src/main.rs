use petgraph::algo::has_path_connecting;
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

    // Ask the library whether a path connects a and e.
    let connected = has_path_connecting(&g, "a", "e", None);
    println!("{}", connected);
}
