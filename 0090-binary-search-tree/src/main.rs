struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

fn insert(root: Option<Box<Node>>, value: i32) -> Box<Node> {
    match root {
        None => Box::new(Node { value, left: None, right: None }),
        Some(mut node) => {
            if value < node.value {
                node.left = Some(insert(node.left.take(), value));
            } else {
                node.right = Some(insert(node.right.take(), value));
            }
            node
        }
    }
}

fn inorder(root: &Option<Box<Node>>, out: &mut Vec<String>) {
    if let Some(node) = root {
        inorder(&node.left, out);
        out.push(node.value.to_string());
        inorder(&node.right, out);
    }
}

fn main() {
    let mut root: Option<Box<Node>> = None;
    for v in [5, 3, 8, 1, 4] {
        root = Some(insert(root.take(), v));
    }
    let mut out = Vec::new();
    inorder(&root, &mut out);
    println!("{}", out.join(" "));
}
