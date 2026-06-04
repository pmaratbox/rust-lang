enum Nested {
    Leaf(i32),
    List(Vec<Nested>),
}

fn flatten(node: &Nested, out: &mut Vec<i32>) {
    match node {
        Nested::Leaf(v) => out.push(*v),
        Nested::List(items) => {
            for item in items {
                flatten(item, out);
            }
        }
    }
}

fn main() {
    use Nested::*;
    let data = List(vec![
        Leaf(1),
        List(vec![Leaf(2), List(vec![Leaf(3), Leaf(4)])]),
        Leaf(5),
    ]);
    let mut out = Vec::new();
    flatten(&data, &mut out);
    let parts: Vec<String> = out.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
