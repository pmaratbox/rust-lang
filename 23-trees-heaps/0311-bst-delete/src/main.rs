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

fn min_value(node: &Node) -> i64 {
    let mut cur = node;
    while let Some(l) = &cur.left {
        cur = l;
    }
    cur.val
}

fn delete(root: &mut Option<Box<Node>>, val: i64) {
    if let Some(n) = root {
        if val < n.val {
            delete(&mut n.left, val);
        } else if val > n.val {
            delete(&mut n.right, val);
        } else {
            match (n.left.take(), n.right.take()) {
                (None, None) => *root = None,
                (Some(l), None) => *root = Some(l),
                (None, Some(r)) => *root = Some(r),
                (Some(l), Some(r)) => {
                    n.left = Some(l);
                    n.right = Some(r);
                    let succ = min_value(n.right.as_ref().unwrap());
                    n.val = succ;
                    delete(&mut n.right, succ);
                }
            }
        }
    }
}

fn inorder(node: &Option<Box<Node>>, out: &mut Vec<i64>) {
    if let Some(n) = node {
        inorder(&n.left, out);
        out.push(n.val);
        inorder(&n.right, out);
    }
}

fn main() {
    let mut root = None;
    for v in [5, 3, 8, 1, 4] {
        insert(&mut root, v);
    }
    delete(&mut root, 3);
    let mut out = Vec::new();
    inorder(&root, &mut out);
    let parts: Vec<String> = out.iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
