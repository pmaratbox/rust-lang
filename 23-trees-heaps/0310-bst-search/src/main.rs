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

fn search(root: &Option<Box<Node>>, val: i64) -> bool {
    match root {
        None => false,
        Some(n) => {
            if val == n.val {
                true
            } else if val < n.val {
                search(&n.left, val)
            } else {
                search(&n.right, val)
            }
        }
    }
}

fn main() {
    let mut root = None;
    for v in [5, 3, 8, 1, 4] {
        insert(&mut root, v);
    }
    let a = if search(&root, 4) { "yes" } else { "no" };
    let b = if search(&root, 6) { "yes" } else { "no" };
    println!("{} {}", a, b);
}
