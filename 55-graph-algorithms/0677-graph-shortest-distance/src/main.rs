use petgraph::algo::dijkstra;
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

    // Let petgraph's Dijkstra compute the weighted shortest-path distance a -> e.
    let dist = dijkstra(&g, "a", Some("e"), |e| *e.2);
    println!("{}", dist[&"e"]);
}
