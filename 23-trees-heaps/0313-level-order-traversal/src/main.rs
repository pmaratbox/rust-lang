use std::collections::VecDeque;

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

fn level_order(root: &Option<Box<Node>>) -> Vec<i64> {
    let mut out = Vec::new();
    let mut queue: VecDeque<&Node> = VecDeque::new();
    if let Some(n) = root {
        queue.push_back(n);
    }
    while let Some(n) = queue.pop_front() {
        out.push(n.val);
        if let Some(l) = &n.left {
            queue.push_back(l);
        }
        if let Some(r) = &n.right {
            queue.push_back(r);
        }
    }
    out
}

fn main() {
    let mut root = None;
    for v in [5, 3, 8, 1, 4] {
        insert(&mut root, v);
    }
    let parts: Vec<String> = level_order(&root).iter().map(|v| v.to_string()).collect();
    println!("{}", parts.join(" "));
}
