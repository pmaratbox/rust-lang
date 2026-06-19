use petgraph::algo::astar;
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

    // A* gives the actual shortest path (not just the distance) from a to e.
    // Edge weights come from the tuple element e.2; the zero heuristic makes
    // A* behave like Dijkstra, returning the unique cost-4 route.
    let (_cost, path) = astar(&g, "a", |n| n == "e", |e| *e.2, |_| 0).unwrap();
    println!("{}", path.join("-"));
}
