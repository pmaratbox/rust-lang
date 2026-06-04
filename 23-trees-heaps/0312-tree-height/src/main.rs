struct Node {
    val: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn insert(root: &mut Option<Box<Node>>, val: i64) {
    match root {
        None => *root = Some(Box::new(Node { val, left: None, right: None })),
        Some(n) => {
            if val < n.val {
                insert(&mut n.left, val);
            } else {
                insert(&mut n.right, val);
            }
        }
    }
}

fn height(node: &Option<Box<Node>>) -> i64 {
    match node {
        None => 0,
        Some(n) => 1 + height(&n.left).max(height(&n.right)),
    }
}

fn main() {
    let mut root = None;
    for v in [5, 3, 8, 1, 4] {
        insert(&mut root, v);
    }
    println!("{}", height(&root));
}
