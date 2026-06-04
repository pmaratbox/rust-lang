struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn leaf(val: i64) -> Box<Node> {
        Box::new(Node { val, left: None, right: None })
    }
    fn branch(val: i64, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Box<Node> {
        Box::new(Node { val, left, right })
    }
}

fn valid(node: &Option<Box<Node>>, low: i64, high: i64) -> bool {
    match node {
        None => true,
        Some(n) => {
            n.val > low
                && n.val < high
                && valid(&n.left, low, n.val)
                && valid(&n.right, n.val, high)
        }
    }
}

fn main() {
    // Correct BST
    //       5
    //      / \
    //     3   8
    //    / \
    //   1   4
    let good = Node::branch(
        5,
        Some(Node::branch(3, Some(Node::leaf(1)), Some(Node::leaf(4)))),
        Some(Node::leaf(8)),
    );

    // Invalid: 6 in the left subtree of 5 is out of place
    let bad = Node::branch(
        5,
        Some(Node::branch(3, Some(Node::leaf(1)), Some(Node::leaf(6)))),
        Some(Node::leaf(8)),
    );

    let a = if valid(&Some(good), i64::MIN, i64::MAX) { "yes" } else { "no" };
    let b = if valid(&Some(bad), i64::MIN, i64::MAX) { "yes" } else { "no" };
    println!("{} {}", a, b);
}
