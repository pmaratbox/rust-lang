use petgraph::algo::toposort;
use petgraph::graphmap::DiGraphMap;

fn main() {
    // Fixed DAG: a->b, b->c, a->c, c->d, d->e (unique topological order).
    let dag: DiGraphMap<&str, ()> = DiGraphMap::from_edges(&[
        ("a", "b"),
        ("b", "c"),
        ("a", "c"),
        ("c", "d"),
        ("d", "e"),
    ]);

    // Let petgraph compute the topological order.
    let order = toposort(&dag, None).unwrap();
    let names: Vec<&str> = order.into_iter().collect();
    println!("{}", names.join(","));
}
