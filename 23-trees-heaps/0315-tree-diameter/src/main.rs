struct Node {
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn leaf() -> Box<Node> {
        Box::new(Node { left: None, right: None })
    }
    fn branch(left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { left, right })
    }
}

// Returns height in edges; updates best diameter (in edges) seen so far.
fn diameter(node: &Option<Box<Node>>, best: &mut i64) -> i64 {
    match node {
        None => -1,
        Some(n) => {
            let lh = diameter(&n.left, best);
            let rh = diameter(&n.right, best);
            *best = (*best).max(lh + rh + 2);
            1 + lh.max(rh)
        }
    }
}

fn main() {
    //        root
    //        /  \
    //       A    B
    //      / \
    //     C   D
    let tree = Node::branch(
        Some(Node::branch(Some(Node::leaf()), Some(Node::leaf()))),
        Some(Node::leaf()),
    );
    let mut best = 0;
    diameter(&Some(tree), &mut best);
    println!("{}", best);
}
