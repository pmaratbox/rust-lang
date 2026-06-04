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

fn preorder(root: &Option<Box<Node>>) -> Vec<i64> {
    let mut out = Vec::new();
    let mut stack: Vec<&Node> = Vec::new();
    if let Some(n) = root {
        stack.push(n);
    }
    while let Some(n) = stack.pop() {
        out.push(n.val);
        if let Some(r) = &n.right {
            stack.push(r);
        }
        if let Some(l) = &n.left {
            stack.push(l);
        }
    }
    out
}

fn main() {
    let mut root = None;
    for v in [5, 3, 8, 1, 4] {
        insert(&mut root, v);
    }
    let parts: Vec<String> = preorder(&root).iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
